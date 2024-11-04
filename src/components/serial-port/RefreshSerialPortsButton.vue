<template>
  <button @click="refrehSerialPorts"></button>
</template>

<script lang="ts" setup>
import { useAppStore } from "@/stores/app";

const refrehSerialPorts = () => {
  const app = useAppStore();

  console.warn("Not implemented yet!");
  app.getSerialPorts();
  console.error("Cant reach this but why!");
};
</script>
