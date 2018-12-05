pub mod chokepoints;
pub mod classification;
pub mod debug_objects;
pub mod diff_all;
pub mod diff_worlds;
pub mod edit;
pub mod edit_mode;
pub mod floodfill;
pub mod follow;
pub mod geom_validation;
pub mod hider;
pub mod layers;
pub mod logs;
pub mod neighborhood_summary;
pub mod search;
pub mod show_activity;
pub mod show_owner;
pub mod show_route;
pub mod sim_controls;
pub mod steep;
pub mod time_travel;
pub mod turn_cycler;
pub mod warp;

use abstutil;
use abstutil::WeightedUsizeChoice;
use downcast::Any;
use ezgui::{Color, GfxCtx, WrappedWizard};
use map_model::{IntersectionID, Map};
use objects::{Ctx, ID};
use sim::{ABTest, Neighborhood, NeighborhoodBuilder, OriginDestination, Scenario, Tick};
use ui::PluginCtx;

pub trait Plugin: Any {
    fn color_for(&self, _obj: ID, _ctx: Ctx) -> Option<Color> {
        None
    }

    fn draw(&self, _g: &mut GfxCtx, _ctx: Ctx) {}

    // True if active
    fn event(&mut self, _ctx: PluginCtx) -> bool {
        false
    }

    // TODO Such hacks
    fn new_event(&mut self, _ctx: &mut PluginCtx) -> bool {
        false
    }
}

downcast!(Plugin);

// TODO Further refactoring should be done, but at least group these here to start.
// General principles are to avoid actually deserializing the objects unless needed.

pub fn choose_neighborhood(map: &Map, wizard: &mut WrappedWizard, query: &str) -> Option<String> {
    let map_name = map.get_name().to_string();
    let gps_bounds = map.get_gps_bounds().clone();
    // Load the full object, since various plugins visualize the neighborhood when menuing over it
    wizard
        .choose_something::<Neighborhood>(
            query,
            Box::new(move || Neighborhood::load_all(&map_name, &gps_bounds)),
        ).map(|(n, _)| n)
}

pub fn load_neighborhood_builder(
    map: &Map,
    wizard: &mut WrappedWizard,
    query: &str,
) -> Option<NeighborhoodBuilder> {
    let map_name = map.get_name().to_string();
    wizard
        .choose_something::<NeighborhoodBuilder>(
            query,
            Box::new(move || abstutil::load_all_objects("neighborhoods", &map_name)),
        ).map(|(_, n)| n)
}

pub fn load_scenario(map: &Map, wizard: &mut WrappedWizard, query: &str) -> Option<Scenario> {
    let map_name = map.get_name().to_string();
    wizard
        .choose_something::<Scenario>(
            query,
            Box::new(move || abstutil::load_all_objects("scenarios", &map_name)),
        ).map(|(_, s)| s)
}

pub fn choose_scenario(map: &Map, wizard: &mut WrappedWizard, query: &str) -> Option<String> {
    let map_name = map.get_name().to_string();
    wizard
        .choose_something::<String>(
            query,
            Box::new(move || abstutil::list_all_objects("scenarios", &map_name)),
        ).map(|(n, _)| n)
}

// TODO Implicitly need a blank edits entry
pub fn choose_edits(map: &Map, wizard: &mut WrappedWizard, query: &str) -> Option<String> {
    let map_name = map.get_name().to_string();
    wizard
        .choose_something::<String>(
            query,
            Box::new(move || abstutil::list_all_objects("edits", &map_name)),
        ).map(|(n, _)| n)
}

pub fn load_ab_test(map: &Map, wizard: &mut WrappedWizard, query: &str) -> Option<ABTest> {
    let map_name = map.get_name().to_string();
    wizard
        .choose_something::<ABTest>(
            query,
            Box::new(move || abstutil::load_all_objects("ab_tests", &map_name)),
        ).map(|(_, t)| t)
}

pub fn input_tick(wizard: &mut WrappedWizard, query: &str) -> Option<Tick> {
    wizard.input_something(query, None, Box::new(|line| Tick::parse(&line)))
}

pub fn input_weighted_usize(
    wizard: &mut WrappedWizard,
    query: &str,
) -> Option<WeightedUsizeChoice> {
    wizard.input_something(
        query,
        None,
        Box::new(|line| WeightedUsizeChoice::parse(&line)),
    )
}

// TODO Validate the intersection exists? Let them pick it with the cursor?
pub fn choose_intersection(wizard: &mut WrappedWizard, query: &str) -> Option<IntersectionID> {
    wizard.input_something(
        query,
        None,
        Box::new(|line| {
            usize::from_str_radix(&line, 10)
                .ok()
                .map(|id| IntersectionID(id))
        }),
    )
}

pub fn choose_origin_destination(
    map: &Map,
    wizard: &mut WrappedWizard,
    query: &str,
) -> Option<OriginDestination> {
    let neighborhood = "Neighborhood";
    let border = "Border intersection";
    if wizard.choose_string(query, vec![neighborhood, border])? == neighborhood {
        choose_neighborhood(map, wizard, query).map(|n| OriginDestination::Neighborhood(n))
    } else {
        choose_intersection(wizard, query).map(|i| OriginDestination::Border(i))
    }
}
