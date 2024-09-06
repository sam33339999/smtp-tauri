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
  label: {
    type: String,
    default: "content",
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
  <Panel :header="props.label">
    <div style="display: flex; flex-direction: column;">
      <label class="mb-2" for="from">從(from)</label>
      <InputText id="from" v-model="sendForm.from"></InputText>
      
      <label class="my-2" for="to">到(to)</label><span class="text-gray-400 text-sm">增加標籤的話請自行添加+tag, 舉例像是: info+test+hello@gmail.com</span>
      <InputText id="to" v-model="sendForm.to"></InputText>

      <label class="my-2" for="subject">主旨(subject)</label>
      <InputText id="subject" v-model="sendForm.subject"></InputText>

      <label class="my-2" for="html">內文(html)</label>
      <Editor id="html" v-model="sendForm.html" editorStyle="height: 320px" />
    </div>
  </Panel>
</template>

<style scoped>
</style>