use crate::{
    framework::{self, DebugMode},
    managers::{
        self, debugger, physics::{
            get_ray_intersaction_position, is_ray_intersecting, CollisionGroups,
            ObjectBodyParameters, RenderRay,
        }, render, ui::Vec3Inspector
    },
    math_utils::deg_vec_to_rad,
};
use glam::{Quat, Vec3};
use rapier3d::{geometry::InteractionGroups, pipeline::QueryFilter};

use super::{gen_object_id, Object, ObjectGroup, Transform};

#[derive(Debug)]
pub struct Ray {
    name: String,
    transform: Transform,
    parent_transform: Option<Transform>,
    children: Vec<Box<dyn Object>>,
    id: u128,
    groups: Vec<ObjectGroup>,
    direction: Vec3,
    mask: CollisionGroups,
    inspector: Vec3Inspector
}

impl Ray {
    pub fn new(name: &str, direction: Vec3, mask: Option<CollisionGroups>) -> Self {
        let mask = match mask {
            Some(mask) => mask,
            None => CollisionGroups::full(), // maybe use all if this won't work?
        };

        Ray {
            transform: Transform::default(),
            children: vec![],
            name: name.to_string(),
            parent_transform: None,
            id: gen_object_id(),
            groups: vec![],
            direction,
            mask,
            inspector: Vec3Inspector::default()
        }
    }
}

impl Object for Ray {
    fn start(&mut self) {}

    fn update(&mut self) {}

    fn children_list(&self) -> &Vec<Box<dyn Object>> {
        &self.children
    }

    fn children_list_mut(&mut self) -> &mut Vec<Box<dyn Object>> {
        &mut self.children
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn object_type(&self) -> &str {
        "Ray"
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn local_transform(&self) -> Transform {
        self.transform
    }

    fn set_local_transform(&mut self, transform: Transform) {
        self.transform = transform
    }

    fn parent_transform(&self) -> Option<Transform> {
        self.parent_transform
    }

    fn set_parent_transform(&mut self, transform: Transform) {
        self.parent_transform = Some(transform);
    }

    fn set_body_parameters(&mut self, _rigid_body: Option<ObjectBodyParameters>) {
        debugger::error("failed to call set_body_parameters!\nRay objects can't have bodies");
    }

    fn body_parameters(&self) -> Option<ObjectBodyParameters> {
        None
    }

    fn object_id(&self) -> &u128 {
        &self.id
    }

    fn inspector_ui(&mut self, ui: &mut egui_glium::egui_winit::egui::Ui) {
        ui.heading("Ray parameters");
        ui.label("direction:");
        if let Some(new_dir) = managers::ui::draw_vec3_editor_inspector(ui, &mut self.inspector, &self.direction, true) {
            self.direction = new_dir;
        }
        ui.label(format!("which is {} if we take object rotation into account", self.rotated_direction()));
    }

    fn groups_list(&mut self) -> &mut Vec<super::ObjectGroup> {
        &mut self.groups
    }

    fn debug_render(&self) {
        if let DebugMode::Full = framework::get_debug_mode() {
            render::add_ray_to_draw(RenderRay {
                origin: self.global_transform().position,
                direction: self.rotated_direction(),
            });
        }
    }
}

impl Ray {
    pub fn is_intersecting(&self) -> bool {
        let global_transform = self.global_transform();
        let rotated_direction = self.rotated_direction();

        let toi = rotated_direction.distance(global_transform.position);
        let query_filter = QueryFilter::new().groups(InteractionGroups::new(
            CollisionGroups::Group1.bits().into(),
            self.mask.bits().into(),
        ));

        let ray = rapier3d::geometry::Ray::new(
            global_transform.position.into(),
            rotated_direction.into(),
        );

        //dbg!(toi);
        //dbg!(rotated_direction.normalize());
        //dbg!(query_filter.groups);

        is_ray_intersecting(ray, toi, query_filter)
    }

    pub fn intersection_position(&self) -> Option<Vec3> {
        let global_transform = self.global_transform();
        let rotated_direction = self.rotated_direction();

        let toi = rotated_direction.distance(global_transform.position);
        let query_filter = QueryFilter::new().groups(InteractionGroups::new(
            CollisionGroups::Group1.bits().into(),
            self.mask.bits().into(),
        ));

        let ray = rapier3d::geometry::Ray::new(
            global_transform.position.into(),
            rotated_direction.into(),
        );

        get_ray_intersaction_position(ray, toi, query_filter)
    }

    fn rotated_direction(&self) -> Vec3 {
        let global_transform = self.global_transform();
        let rotation = deg_vec_to_rad(global_transform.rotation);
        let rotation_quat =
            Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
        //let direction_quat = Quat::from_euler(glam::EulerRot::XYZ, self.direction.x, self.direction.y, self.direction.z);
        //let rotated_direction = direction_quat
        //    .mul_vec3(global_transform.rotation);
        let rotated_direction = rotation_quat.mul_vec3(self.direction);

        rotated_direction
    }
}
