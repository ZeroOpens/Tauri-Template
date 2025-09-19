<template>
  <Index />
</template>

<script setup lang="ts">
  defineOptions({name: 'App'})
  import Index from '@/pages/index.vue'

  
  // window 禁用鼠标右键和F12键
  // window.addEventListener('contextmenu', e => e.preventDefault());
  // window.addEventListener('keydown', e => e.key === 'F12' && e.preventDefault());
  
</script>

<style scoped>

</style>
 