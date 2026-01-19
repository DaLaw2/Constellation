<template>
  <div class="main-layout">
    <TopBar />
    <div class="content-area">
      <LeftPanel />
      <div class="right-panel">
        <FileList />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import TopBar from './TopBar.vue'
import LeftPanel from './LeftPanel.vue'
import FileList from '../FileExplorer/FileList.vue'
</script>

<style scoped>
.main-layout {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-area {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.right-panel {
  flex: 1;
  overflow: hidden;
}
</style>
