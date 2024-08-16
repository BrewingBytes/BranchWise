import { test, expect, vi } from "vitest";
import { createApp } from "vue";
import App from "../App.vue";

test('App is mounted', () => {
    document.body.innerHTML =
        '<div id="app">' +
        '</div>';


    import('../main').then(() => {
        const appElement = document.getElementById('app');
        expect(appElement).toBeTruthy();
    });
});

test('Plugins are registered', () => {
    import('../plugins').then(({ default: registerPlugins }) => {
        const app = createApp(App);
        vi.spyOn(app, 'use');

        expect(registerPlugins(app)).toHaveBeenCalled();

        expect(app.use).toHaveBeenCalled();

        import('../plugins/vuetify').then(({ default: vuetify }) => {
            expect(app.use).toHaveBeenCalledWith(vuetify);
        });
    });
});

test('Vuetify is registered', () => {
    import('../plugins/vuetify').then(({ default: vuetify }) => {
        const app = createApp(App);
        vi.spyOn(app, 'use');

        expect(vuetify).toBeTruthy();

        expect(app.use).toHaveBeenCalledWith(vuetify);

        expect(vuetify.icons.defaultSet).toBe('mdi');
    });
});
