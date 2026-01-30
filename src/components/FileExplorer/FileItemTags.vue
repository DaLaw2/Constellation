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
import { ref, watch, onMounted } from 'vue'
import { useTagsStore } from '@/stores/tags'
import { useItemsStore } from '@/stores/items'
import TagCell from '../TagManagement/TagCell.vue'
import type { FileEntry, Tag } from '@/types'

interface FileItemTagsProps {
  entry: FileEntry
}

const props = defineProps<FileItemTagsProps>()

const tagsStore = useTagsStore()
const itemsStore = useItemsStore()

const itemTags = ref<Tag[]>([])
const itemId = ref<number | null>(null)

const tagGroups = tagsStore.tagGroups
const allTags = tagsStore.tags

// Load tags when entry changes
watch(
  () => props.entry.path,
  async (newPath) => {
    try {
      const item = await itemsStore.getItemByPath(newPath)
      if (item) {
        itemId.value = item.id
        const tags = await itemsStore.getTagsForItem(item.id)
        itemTags.value = tags
      } else {
        itemId.value = null
        itemTags.value = []
      }
    } catch (e) {
      console.error('Failed to load item tags:', e)
      itemTags.value = []
    }
  },
  { immediate: true }
)

async function handleTagsUpdate(tagIds: number[]) {
  try {
    // Create item if it doesn't exist
    if (itemId.value === null) {
      const newId = await itemsStore.createItem(
        props.entry.path,
        props.entry.is_directory,
        props.entry.size,
        props.entry.modified_time
      )
      itemId.value = newId
    }

    // Update tags
    await itemsStore.updateItemTags(itemId.value, tagIds)
    const tags = await itemsStore.getTagsForItem(itemId.value)
    itemTags.value = tags
  } catch (e) {
    console.error('Failed to update tags:', e)
  }
}

async function handleCreateTag(groupId: number, value: string) {
  try {
    const newTagId = await tagsStore.createTag(groupId, value)
    if (itemId.value !== null) {
      const currentIds = itemTags.value.map(t => t.id)
      await itemsStore.updateItemTags(itemId.value, [...currentIds, newTagId])
      const tags = await itemsStore.getTagsForItem(itemId.value)
      itemTags.value = tags
    }
  } catch (e) {
    console.error('Failed to create tag:', e)
  }
}

// Load tag groups and tags on mount
onMounted(() => {
  if (tagsStore.tagGroups.length === 0) {
    tagsStore.loadTagGroups()
  }
  if (tagsStore.tags.length === 0) {
    tagsStore.loadTags()
  }
})
</script>

<style scoped>
.file-item-tags {
  display: flex;
  align-items: center;
  min-width: 0;
  flex: 1;
}
</style>
