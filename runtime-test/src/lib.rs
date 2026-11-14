#[cfg(test)]
mod tests {
    use wasmtime::component::{Component, Linker, ResourceTable};
    use wasmtime::{Config, Engine, Result, Store};
    use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
    wasmtime::component::bindgen!({ path: "../src/wit", world: "layout-plugin" });
    use exports::gams::layout::layout::{Action, InitScreenRequest, LayoutConfig, ResizeScreenRequest};

    struct State { wasi: WasiCtx, table: ResourceTable }
    impl WasiView for State {
        fn ctx(&mut self) -> WasiCtxView<'_> {
            WasiCtxView { ctx: &mut self.wasi, table: &mut self.table }
        }
    }
    fn layout_config() -> LayoutConfig {
        LayoutConfig { max_areas: 8, max_handles: 8, min_panel_size: 20, handle_half_size: 3 }
    }
    #[test]
    fn screen_initialization_resize_and_invalid_dimensions() -> Result<()> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;
        let component = Component::from_file(&engine, std::env::var("LAYOUT_COMPONENT_PATH")?)?;
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
        let mut store = Store::new(&engine, State { wasi: WasiCtx::builder().build(), table: ResourceTable::new() });
        let bindings = LayoutPlugin::instantiate(&mut store, &component, &linker)?;
        let layout = bindings.gams_layout_layout();
        let initialized = layout.call_init_screen(&mut store, &InitScreenRequest {
            w: 640, h: 480, config: layout_config(), root_content_id: "root".into(),
        })?.unwrap();
        assert_eq!(initialized.document.screen_w, 640);
        assert_eq!(initialized.document.screen_h, 480);
        assert_eq!(initialized.document.areas.len(), 1);
        assert_eq!(initialized.document.areas[0].content_id, "root");
        assert_eq!(initialized.info.action, Action::InitScreen);
        let resized = layout.call_resize_screen(&mut store, &ResizeScreenRequest {
            document: initialized.document, w: 800, h: 600, handle_half_size: 4,
        })?.unwrap();
        assert_eq!(resized.document.screen_w, 800);
        assert_eq!(resized.document.screen_h, 600);
        assert_eq!(resized.info.action, Action::ResizeScreen);
        assert!(layout.call_init_screen(&mut store, &InitScreenRequest {
            w: 0, h: 480, config: layout_config(), root_content_id: "root".into(),
        })?.is_err());
        Ok(())
    }
}
