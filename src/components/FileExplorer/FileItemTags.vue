<template>
  <div class="file-item-tags">
    <TagCell
      :item-tags="itemTags"
      :tag-groups="tagGroups"
      :tags="allTags"
      @update:tags="handleTagsUpdate"
      @create-tag="handleCreateTag"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useTagsStore } from '@/stores/tags'
import { useItemsStore } from '@/stores/items'
import TagCell from '../TagManagement/TagCell.vue'
import type { FileEntry, Tag } from '@/types'

interface FileItemTagsProps {
  entry: FileEntry
  tags?: Tag[]
}

const props = withDefaults(defineProps<FileItemTagsProps>(), {
  tags: () => []
})

const tagsStore = useTagsStore()
const itemsStore = useItemsStore()

const itemId = ref<number | null>(null)

// Reset itemId when entry changes (important for RecycleScroller component reuse)
watch(() => props.entry.path, () => {
  itemId.value = null
})

const tagGroups = computed(() => tagsStore.tagGroups)
const allTags = computed(() => tagsStore.tags)
const itemTags = computed(() => props.tags)

async function handleTagsUpdate(tagIds: number[]) {
  try {
    // Get or create item
    if (!itemId.value) {
      const item = await itemsStore.getItemByPath(props.entry.path)
      if (item) {
        itemId.value = item.id
      } else {
        const newId = await itemsStore.createItem(
          props.entry.path,
          props.entry.is_directory,
          props.entry.size,
          props.entry.modified_time
        )
        itemId.value = newId
      }
    }

    // Update tags (triggers itemTagsVersion increment for cache refresh)
    await itemsStore.updateItemTags(itemId.value, tagIds)
  } catch (e) {
    console.error('Failed to update tags:', e)
  }
}

async function handleCreateTag(groupId: number, value: string) {
  try {
    const newTagId = await tagsStore.createTag(groupId, value)

    // Get or create item
    if (!itemId.value) {
      const item = await itemsStore.getItemByPath(props.entry.path)
      if (item) {
        itemId.value = item.id
      } else {
        const newId = await itemsStore.createItem(
          props.entry.path,
          props.entry.is_directory,
          props.entry.size,
          props.entry.modified_time
        )
        itemId.value = newId
      }
    }

    // Add the new tag (triggers itemTagsVersion increment for cache refresh)
    const currentIds = props.tags.map(t => t.id)
    await itemsStore.updateItemTags(itemId.value, [...currentIds, newTagId])
  } catch (e) {
    console.error('Failed to create tag:', e)
  }
}
</script>

<style scoped>
.file-item-tags {
  display: flex;
  align-items: center;
  min-width: 0;
  flex: 1;
}
</style>
