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

## Grid Virtual Scrolling (Row Grouping)

`RecycleScroller` requires fixed item sizes, which works for list views but not grids where column count varies with container width. The solution is to group items into rows:

### useGridVirtualScroll Composable

```typescript
import { useGridVirtualScroll } from '@/composables'

// Groups items into rows based on container width
const { rows, columnCount } = useGridVirtualScroll(items, {
  minCardWidth: 150,   // or a computed ref for zoom support
  gap: 16,
  containerRef,        // template ref for ResizeObserver
})
```

### Usage with RecycleScroller

```vue
<RecycleScroller
  :items="rows"
  :item-size="rowHeight"
  key-field="id"
  v-slot="{ item: row }"
>
  <div class="grid-row" :style="{ display: 'grid', gridTemplateColumns: `repeat(${columnCount}, 1fr)` }">
    <Card v-for="item in row.items" :key="item.id" :data="item" />
  </div>
</RecycleScroller>
```

### How It Works

1. `ResizeObserver` monitors container width
2. Column count calculated: `floor((width + gap) / (minCardWidth + gap))`
3. Items grouped into rows of N (last row may be partial)
4. Each row has a stable `id` (`row-{startIndex}`) for RecycleScroller
5. `startIndex` field enables computing global index from row-local index (for lightbox, etc.)
6. Supports reactive `minCardWidth`/`gap` (e.g., zoom level changes)

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
