<template>
  <div class="function">
    <div class="function-content">
      <Update/>
    </div>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Function'})
  import Update from '@/components/update.vue'

</script>

<style scoped lang="scss">
.function {
  width: 100%;
  height: 100%;
  margin-top: 50px;
}
</style>
 