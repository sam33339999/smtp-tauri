<script setup>
import { ref } from 'vue';
import { nanoid } from 'nanoid';
import SmtpForm from "./components/SmtpForm.vue";
import SmtpContent from "./components/SmtpContent.vue";
import { invoke } from "@tauri-apps/api/tauri";
import Button from 'primevue/button';
import Divider from 'primevue/divider';
import Message from 'primevue/message';
import Panel from 'primevue/panel';
import ProgressBar from 'primevue/progressbar';

const config = ref({host: "127.0.0.1", port: 1025, username: "", password: ""});
const content = ref({from: "", to: "", subject: "測試", text: "???", html: "<h2><Hello World !!!/h2>"});
const retMessages = ref([]);
const pendingMessages = ref([]);


// 清除等待處理的消息 by identify
const dropPendingMessage = (identify) => {
  pendingMessages.value = pendingMessages.value.filter((msg) => msg.identify !== identify);
};

const sendMail = async () => {
  const identify = nanoid();
  try {
    pendingMessages.value = [{ identify, type: 'info' }, ...pendingMessages.value];

    const result = await invoke("send_mail", {
      smtp: {
        config: {
          host: config.value.host,
          port: parseInt(config.value.port),
          username: config.value.username,
          password: config.value.password,
        },
        content: content.value,
      }
    });

    let msg = {
      type: "success",
      result: result,
    };

    retMessages.value = [msg, ...retMessages.value];
    dropPendingMessage(identify);
  } catch (e) {
    let msg = {
      type: "error",
      result: e,
    };
    console.error(e);
    retMessages.value = [msg, ...retMessages.value];
    dropPendingMessage(identify);
  }
};

const cleanAll = () => {
  retMessages.value = [];
  pendingMessages.value = [];
};

</script>

<template>
  <div class="m-2 h-64 mt-4 flex">
    <div class="mx-2" style="width: 25%">
      <SmtpForm v-model:setting="config"></SmtpForm>
      <Divider />

      <div class="mt-10 w-full flex justify-around ">
        <Button label="寄送(Send)" @click="sendMail"></Button>
        <Button label="清除(Clean)" @click="cleanAll" severity="danger"></Button>
      </div>
      <Divider />
      <div class="mt-5 w-full max-h-96 overflow-scroll h-full ">
        <div class="mt-2" v-for="(msg, idx) of pendingMessages" :key="`pending-${idx}`">
          <Message>{{ msg }}</Message>
        </div>
        <div class="mt-2" v-for="(msg, idx) of retMessages" :key="`ret-${idx}`">
          <Message :severity="msg.type">{{ msg.result }}</Message>
        </div>
      </div>
    </div>

    <div class="mx-2" style="width: 75%">
      <SmtpContent v-model:form="content"></SmtpContent>
      <ProgressBar :value="pendingMessages.length"/>
    </div>

    <!-- <div class="mx-2 h-full min-h-max" style="width: 10%">
      <Panel header="pending" class="h-full bg-red-200">
        
      </Panel>
    </div> -->
  </div>
</template>

<style scoped>

</style>
