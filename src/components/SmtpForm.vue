<script setup>
import { ref, watch } from "vue";
import InputText from 'primevue/inputtext';
import Panel from 'primevue/panel';

const props = defineProps({
  setting: {
    type: Object, // {host: "", port: 587, username: "", password: ""}
    required: true,
  },
});
const emit = defineEmits(['update:setting']);
const localForm = ref({ ...props.setting });

// watch
watch(localForm, (newVal, oldVal) => {
  emit('update:setting', newVal);
}, { deep: true });
</script>

<template>
  <Panel header="setting">
    <div style="display: flex; flex-direction: column;">
      <label for="host">目標主機(target host)</label>
      <InputText id="host" v-model="localForm.host"></InputText>
      <label for="port">目標埠號(target port)</label>
      <InputText id="port" v-model="localForm.port"></InputText>
      <label for="username">帳號(username)</label>
      <InputText id="username" v-model="localForm.username"></InputText>
      <label for="password">密碼(password)</label>
      <InputText id="password" v-model="localForm.password"></InputText>
    </div>
  </Panel>
</template>
