// Utilities
import { defineStore } from 'pinia'
import axios from 'axios'
import { Video } from '../bindings';
import { Ref, computed, ref } from 'vue';

export const useAppStore = defineStore('app', () => {
  const videos:Ref<Video[]>=ref([]);
  const videoCount=computed(()=>{
   return videos.value.length
  });
  async function download() {
    axios.get<Video[]>("api/videos/all").then((res) => {
      videos.value=res.data;
    });
  }
  return {download,videos,videoCount}
})
