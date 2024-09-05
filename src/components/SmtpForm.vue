<script setup>
import { ref, watch } from "vue";
import InputText from 'primevue/inputtext';
import Select from 'primevue/select';
import Panel from 'primevue/panel';

const props = defineProps({
  setting: {
    type: Object, // {host: "", port: 587, username: "", password: ""}
    required: true,
  },
  label: {
    type: String,
    default: "content",
  },
  hasTemplates: {
    type: Array,
  }
});
const emit = defineEmits(['update:setting']);
const localForm = ref({ ...props.setting });
const chosenTemplate = ref(null);

const securityOptions = [
  { label: 'None', value: 'None' }, // other
  { label: 'SSL', value: 'TLS' }, // 465
  { label: 'TLS', value: 'StartTLS' } // 587
];

const useTemplate = () => {
  if (chosenTemplate.value) {
    localForm.value = { ...chosenTemplate.value };
  }
};

// watch
watch(localForm, (newVal, oldVal) => {
  // const port = parseInt(newVal.port);
  // if (port === 587) {
  //   newVal.security = 'StartTLS';
  // } else if (port === 465) {
  //   newVal.security = 'TLS';
  // } else {
  //   newVal.security = 'None';
  // }

  // sync to parent.
  emit('update:setting', newVal);
}, { deep: true });
</script>

<template>
  <Panel :header="props.label">
    <template #header="headerProps">
      
      <div class="flex justify-around">
        <div :class="headerProps.class" class="flex items-center" :id="headerProps.id">{{ props.label }}</div>
        <div class=" ml-5">
          <Select
            size="small"
            v-if="props.hasTemplates"
            :options="props.hasTemplates"
            v-model="chosenTemplate"
            optionLabel="label"
            placeholder="選擇範本"
            @change="useTemplate"
          ></Select>
        </div>
      </div>
      
    </template>
    <div style="display: flex; flex-direction: column;">
      <label for="host">目標主機(target host)</label>
      <InputText size="small" id="host" v-model="localForm.host"></InputText>
      <label for="port">目標埠號(target port)</label>
      <InputText size="small" id="port" v-model="localForm.port" placeholder="25,465(SSL),587(TLS)"></InputText>
      <label for="username">帳號(username)</label>
      <InputText size="small" id="username" v-model="localForm.username"></InputText>
      <label for="password">密碼(password)</label>
      <InputText size="small" id="password" v-model="localForm.password"></InputText>
      <label for="auth_method">驗證方式</label>
      <Select
        id="auth_method"
        :options="securityOptions"
        v-model="localForm.security"
        optionLabel="label"
        optionValue="value"
        placeholder="Select a Security Option"
      />
    </div>
  </Panel>
</template>
