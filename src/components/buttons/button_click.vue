<template>
  <div class="button_click">
    <button :style="buttonStyles">
      <span>{{ text }}</span>
    </button>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'button_click'})
  import { computed } from 'vue'

  // 定义props类型
  interface Props {
    text?: string
    bgColor?: string
    shadowColor?: string
  }

  // 默认值
  const props = withDefaults(defineProps<Props>(), {
    text: 'Button',
    bgColor: '#23c483',
    shadowColor: 'rgba(46, 229, 157, 0.4)',
  })

  // 计算样式
  const buttonStyles = computed(() => ({
    '--bg-color': props.bgColor,
    '--shadow-color': props.shadowColor,
  }))

</script>

<style scoped lang="scss">
  button {
    padding: 0.9rem 1.8rem;
    color: #ffffff;
    border: none;
    border-radius: 45px;
    background-color: var(--bg-color);
    box-shadow: 0px 8px 15px rgba(0, 0, 0, 0.1);
    transition: all 0.3s ease 0s;
    cursor: pointer;

    &:hover {
      box-shadow: 0px 15px 20px var(--shadow-color);
      transform: translateY(-3px);
    }

    &:active {
      transform: translateY(1px);
    }

    span {
      font-size: 1rem;
    }
  }
</style>