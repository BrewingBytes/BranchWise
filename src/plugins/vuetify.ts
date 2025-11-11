import "@mdi/font/css/materialdesignicons.css";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import { aliases, mdi } from "vuetify/iconsets/mdi";
import "vuetify/styles";

export default createVuetify({
	components,
	directives,
	theme: {
		defaultTheme: "dark",
	},
	icons: {
		defaultSet: "mdi",
		aliases,
		sets: {
			mdi
		}
	}
});
