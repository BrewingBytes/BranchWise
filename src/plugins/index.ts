import { App } from "vue";

import pinia from "@/plugins/pinia";
import vuetify from "@/plugins/vuetify";
import router from "@/router";

export default function (app: App) {
	app.use(vuetify).use(router).use(pinia);
}
