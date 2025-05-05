// 组合式写法(推荐)
import { defineStore } from "pinia";
import {reactive} from 'vue'

export const useCountStore = defineStore("count", ()=>{
  // 数据
  const talkList = reactive(
    JSON.parse(localStorage.getItem('talkList') as string) || []
  )

  // 存储数据
  async function getTalk() {
        talkList.unshift({
          id: Date.now(),
          tltle: "pinia数据"
        });
  }

    return{talkList, getTalk}
});