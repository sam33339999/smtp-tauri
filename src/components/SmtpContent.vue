<script setup>
import { ref, watch } from "vue";
import Editor from 'primevue/editor';
import InputText from 'primevue/inputtext';
import Panel from 'primevue/panel';

const props = defineProps({
  form: {
    type: Object, // {from: "", to: "", subject: "", text: "", html: ""}
    required: true,
  },
});
const emit = defineEmits(['update:form']);
const sendForm = ref({ ...props.form });

// watch
watch(sendForm, (newVal, oldVal) => {
  emit('update:form', newVal);
}, { deep: true });
</script>

<template>
  <Panel>

    <template #header="props">
      <span class="w-full" :class="props.class">content</span>
    </template>

    <div style="display: flex; flex-direction: column;">
      <label for="from">從(from)</label>
      <InputText id="from" v-model="sendForm.from"></InputText>
      
      <label for="to">到(to)</label>
      <InputText id="to" v-model="sendForm.to"></InputText>

      <label for="subject">主旨(subject)</label>
      <InputText id="subject" v-model="sendForm.subject"></InputText>

      <label for="html">內文(html)</label>
      <Editor id="html" v-model="sendForm.html" editorStyle="height: 320px" />
    </div>
  </Panel>
</template>

<style scoped>
.p-panel-header {
  background-color: red !important;
}

/* * {
  background-color: lightgray !important;
} */
</style>