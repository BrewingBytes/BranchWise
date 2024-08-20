import { App } from "vue";

import vuetify from "./vuetify";
import router from "../router";
import pinia from "./pinia";

export default function (app: App) {
    app.use(vuetify).use(router).use(pinia);
}
