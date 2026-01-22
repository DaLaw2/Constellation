# Vue 3 Virtual Scrolling

## vue-virtual-scroller

### Installation

```bash
pnpm add vue-virtual-scroller
```

### Basic Usage

```vue
<template>
  <RecycleScroller
    class="scroller"
    :items="items"
    :item-size="48"
    key-field="id"
    v-slot="{ item }"
  >
    <div class="file-item">
      {{ item.name }}
    </div>
  </RecycleScroller>
</template>

<script setup lang="ts">
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'

const items = ref<FileItem[]>([])
</script>

<style>
.scroller {
  height: 100%;
}
</style>
```

### Performance Tips

1. **Fixed Item Height**: Use fixed `item-size` for best performance
2. **Use shallowRef**: For large lists, use `shallowRef` instead of `ref`
3. **Key Field**: Always specify `key-field` for proper recycling

```typescript
import { shallowRef } from 'vue'

// Better performance for large lists
const items = shallowRef<FileItem[]>([])

// Update requires replacing entire array
items.value = [...newItems]
```

### Dynamic Height (if needed)

```vue
<DynamicScroller
  :items="items"
  :min-item-size="32"
  key-field="id"
>
  <template #default="{ item, index, active }">
    <DynamicScrollerItem
      :item="item"
      :active="active"
      :data-index="index"
    >
      <div class="file-item">{{ item.name }}</div>
    </DynamicScrollerItem>
  </template>
</DynamicScroller>
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Item count | 10,000+ items |
| Scroll FPS | 60 FPS |
| Item height | Fixed 48px (recommended) |
| Memory | Only visible items + buffer in DOM |

## Lazy Loading Images

```vue
<template>
  <img
    :src="item.thumbnail"
    loading="lazy"
    decoding="async"
  />
</template>
```

## References

- [vue-virtual-scroller](https://github.com/Akryum/vue-virtual-scroller)
- [Vue 3 Performance Guide](https://vuejs.org/guide/best-practices/performance.html)
