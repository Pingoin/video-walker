<template>
  <v-app-bar flat>
    <v-app-bar-title>
      <v-icon icon="mdi-circle-slice-6" />
      <v-btn
      
      @click="load"
      >
        Button
      </v-btn>
      Essentials Preset
    </v-app-bar-title>
  </v-app-bar>
</template>

<script lang="ts" setup>
import { useAppStore } from "@/store/app"

const store = useAppStore();

async function load() {
  store.download();
}

</script>
