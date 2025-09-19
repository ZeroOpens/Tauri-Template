<template>
  <div class="more">
    <loading class="loading"/>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'More'})
  import loading from '@/components/loading.vue'

</script>

<style scoped lang="scss">
  .more {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
 