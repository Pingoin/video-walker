// Utilities
import { defineStore } from 'pinia'
import axios from 'axios'
import { Video,Folder } from '../bindings';
import { Ref, computed, ref } from 'vue';

export const useAppStore = defineStore('app', () => {
  const videos:Ref<Video[]>=ref([]);
  const folders:Ref<Folder[]>=ref([]);
  const videoCount=computed(()=>{
   return videos.value.length
  });
  const folderCount=computed(()=>{
    return folders.value.length
   });
  async function download() {
    axios.get<Video[]>("api/videos/all").then((res) => {
      videos.value=res.data;
    });
    axios.get<Folder[]>("api/folders/all").then((res) => {
      folders.value=res.data;
    });
  }
  return {download,videos,videoCount,folders,folderCount}
})
