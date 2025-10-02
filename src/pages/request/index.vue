<template>
  <div class="request-index">
    <div v-if="isBaseRequestPath" class="nav-buttons">
      <button_click text="前端请求" bgColor="#4CAF50" shadowColor="rgba(76, 175, 80, 0.4)" fontSize="14px" @click="goToFrontendRequest"></button_click>
      <button_click text="后端请求" bgColor="#F44B01" shadowColor="rgba(250,127, 74, 0.4)" fontSize="16px" @click="goToBackendRequest"></button_click>
    </div>
    <router-view />
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'RequestIndex'})
  import { computed } from 'vue'
  import { useRoute, useRouter } from 'vue-router'
  import button_click from '@/components/buttons/button_click.vue'


  const route = useRoute()
  const router = useRouter()
  // 控制显示隐藏
  const isBaseRequestPath = computed(() => route.path === '/request')

  const goToFrontendRequest = () => {
    router.push('/request/frontend')
  }

  const goToBackendRequest = () => {
    router.push('/request/backend')
  }
</script>

<style scoped lang="scss">
.request-index {
  display: flex;
  flex-direction: column;
  align-items: center;

  .nav-buttons {
    display: flex;
    gap: 20px;
  }
}
</style>