use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use crate::{
    automaton::Automaton,
    cellang,
    graph::{Graph, Node},
    saved_state::SavedState,
    vec2::Vector2,
};
use macroquad::prelude::*;

use egui_macroquad::egui::{
    self, Color32, ComboBox, FontDefinitions, FontFamily, FontSelection, Grid, Id, Pos2, Response,
    Sense, Separator, Stroke, Ui, Vec2, Widget, WidgetWithState,
};
use macroquad::prelude::*;
use serde::{Serialize, Serializer};

pub struct App {
    automaton: Automaton,
    offset: Vector2,
    zoom: f32,
    pub selected: Vec<usize>,
    prev_mouse_position: Vector2,
    dragging_connection: Option<usize>,
    playing: bool,
    adding_state: bool,
    adding_type: String,
    ui_hovering: bool,
    showing_code: bool,
    code: String,
    clipboard: Option<Graph>,
    box_select: Option<Vector2>,
}

impl App {
    pub fn new(automaton: Automaton) -> Self {
        Self {
            automaton,
            offset: Vector2::new(100.0, 100.0),
            zoom: 1.0,
            selected: vec![],
            prev_mouse_position: Vector2::zero(),
            dragging_connection: None,
            playing: false,
            adding_state: false,
            adding_type: String::new(),
            ui_hovering: false,
            showing_code: true,
            code: String::new(),
            clipboard: None,
            box_select: None,
        }
    }

    fn world_to_screen_coord(&self, vec: Vector2) -> Vector2 {
        (vec + self.offset) * self.zoom
    }

    fn screen_to_world_coord(&self, vec: Vector2) -> Vector2 {
        (self.offset * self.zoom - vec) / (-1.0 * self.zoom)
    }

    pub async fn mainloop(&mut self) {
        if self.playing {
            if get_time() % 0.5 < get_frame_time() as f64 {
                self.automaton.step()
            }
        }
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        // Process keys, mouse etc.

        if !self.ui_hovering {
            let mut hovering = None;

            let (m_x, m_y) = mouse_position();

            for (i, node) in self.automaton.graph.nodes.iter().enumerate() {
                if (self.world_to_screen_coord(node.position) - Vector2::new(m_x, m_y)).length()
                    < 30.0 * self.zoom
                {
                    hovering = Some(i);
                }
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(hovering) = hovering {
                    if is_key_down(KeyCode::LeftShift) {
                        if !self.selected.contains(&hovering) {
                            self.selected.push(hovering)
                        }
                    } else {
                        self.selected = vec![hovering];
                    }
                } else {
                    self.selected = vec![]
                }
            }

            if is_mouse_button_down(MouseButton::Left) {
                for selected in &self.selected {
                    self.automaton.graph[*selected].position -=
                        self.prev_mouse_position - Vector2::from(mouse_position());
                }
            }

            if is_mouse_button_pressed(MouseButton::Right) {
                if let Some(hovering) = hovering {
                    self.dragging_connection = Some(hovering)
                }
            }

            if is_mouse_button_released(MouseButton::Right) {
                if let Some(dragging_connection) = self.dragging_connection {
                    if let Some(hovering) = hovering {
                        if !self.automaton.graph[hovering]
                            .edges
                            .contains(&dragging_connection)
                        {
                            self.automaton.graph.add_edge(hovering, dragging_connection);
                        } else {
                            self.automaton
                                .graph
                                .remove_edge(hovering, dragging_connection);
                        }
                    };
                };
                self.dragging_connection = None
            }

            let (scroll_x, scroll_y) = mouse_wheel();
            if is_key_down(KeyCode::LeftShift) {
                self.zoom += -0.1 * scroll_y;
            } else {
                self.offset.x -= scroll_x * 10.0;
                self.offset.y -= scroll_y * 10.0;
            }

            if is_key_pressed(KeyCode::A) {
                self.automaton.graph.add_node(Node::new(
                    self.adding_state,
                    self.adding_state,
                    vec![],
                    self.screen_to_world_coord(Vector2::from(mouse_position())),
                    self.adding_type.clone(),
                ));
            }

            if is_key_pressed(KeyCode::Delete) {
                while let Some(selected) = self.selected.pop() {
                    self.automaton.graph.remove_node(selected);

                    let len = self.automaton.graph.nodes.len();
                    self.selected = self
                        .selected
                        .iter()
                        .map(|a| if *a == len { selected } else { *a })
                        .collect();
                }
            }

            if is_key_down(KeyCode::LeftControl) {
                if is_key_pressed(KeyCode::C) {
                    self.clipboard = Some(self.automaton.graph.copy(&self.selected));
                }

                if is_key_pressed(KeyCode::V) {
                    // // paste
                    self.selected = vec![];
                    let len = self.automaton.graph.nodes.len();
                    if let Some(clipboard) = self.clipboard.clone() {
                        for node in clipboard.nodes {
                            let new_node = Node::new(
                                node.read,
                                node.write,
                                node.edges.iter().map(|a| a + len).collect(),
                                node.position + Vector2::new(50.0, 50.0),
                                node.ruleset.clone(),
                            );

                            self.automaton.graph.add_node(new_node);
                            self.selected.push(self.automaton.graph.nodes.len() - 1)
                        }
                    }
                }
            }

            if let None = hovering {
                if is_mouse_button_pressed(MouseButton::Left) {
                    self.box_select = Some(Vector2::from(mouse_position()));
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(box_drag_start) = self.box_select {
                let rect = find_rect(
                    self.screen_to_world_coord(box_drag_start.into()),
                    self.screen_to_world_coord(Vector2::from(mouse_position())),
                );

                let x_1 = rect.x;
                let y_1 = rect.y;
                let x_2 = rect.w + rect.x;
                let y_2 = rect.h + rect.y;

                if !is_key_down(KeyCode::LeftShift) {
                    self.selected = vec![];
                }
                self.box_select = None;

                for (i, position) in self
                    .automaton
                    .graph
                    .nodes
                    .iter()
                    .map(|a| a.position)
                    .enumerate()
                {
                    if position.x >= x_1
                        && position.x <= x_2
                        && position.y >= y_1
                        && position.y <= y_2
                    {
                        self.selected.push(i)
                    }
                }

                self.box_select = None;
            }
        }

        self.prev_mouse_position = Vector2::from(mouse_position());
        // gui
        egui_macroquad::ui(|egui_ctx| {
            egui_ctx.set_pixels_per_point(1.0);
            egui::TopBottomPanel::new(egui::panel::TopBottomSide::Top, Id::new("top panel")).show(
                egui_ctx,
                |ui| {
                    Grid::new("top panel gird").show(ui, |ui| {
                        if ui.checkbox(&mut self.playing, "playing").clicked() {
                            self.automaton.rules = HashMap::new();

                            for line in self.code.split("\n") {
                                if let Ok(parsed) = cellang::expr_parser::ruleset(&line) {
                                    self.automaton.rules.insert(parsed.name.clone(), parsed);
                                } else {
                                    println!("unable to parse {:?}", line);
                                }
                            }
                        }
                        ui.add(Separator::default().vertical());
                        if ui.button("save").clicked() {
                            self.save_graph();
                        }
                        if ui.button("load").clicked() {
                            self.load_graph();
                        }
                    })
                },
            );
            self.ui_hovering =
                egui::SidePanel::new(egui::panel::Side::Left, Id::new("side pannel"))
                    .resizable(true)
                    .show(egui_ctx, |ui| {
                        ui.label("state");
                        ui.radio_value(&mut self.adding_state, true, "on");
                        ui.radio_value(&mut self.adding_state, false, "off");
                        if ui.button("apply state to selected").clicked() {
                            for selected in &self.selected {
                                self.automaton.graph[*selected].write = self.adding_state;
                            }
                        }
                        ui.separator();
                        egui::ComboBox::from_label("adding type")
                            .selected_text(format!("{}", self.adding_type))
                            .show_ui(ui, |ui| {
                                for rule in self.automaton.rules.keys() {
                                    ui.selectable_value(&mut self.adding_type, rule.clone(), rule);
                                }
                            });
                        ui.end_row();
                        ui.set_width(100.0);
                        ui.separator();

                        self.ui_hovering |= ui.text_edit_multiline(&mut self.code).has_focus();

                        Grid::new("code_grid").show(ui, |ui| {
                            if ui.button("compile code").clicked() {
                                self.automaton.rules = HashMap::new();

                                for line in self.code.split("\n") {
                                    if let Ok(parsed) = cellang::expr_parser::ruleset(&line) {
                                        self.automaton.rules.insert(parsed.name.clone(), parsed);
                                    } else {
                                        println!("unable to parse {:?}", line);
                                    }
                                }
                            }
                            if ui.button("save code").clicked() {
                                self.save_code();
                            }
                            if ui.button("load code").clicked() {
                                self.load_code();
                            }
                            ui.end_row();
                        })
                    })
                    .response
                    .hovered();
        });

        // Draw things before egui
        //
        if let Some(box_corner) = self.box_select {
            let rect = find_rect(box_corner, Vector2::from(mouse_position()));
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                Color::new(0.8, 0.8, 0.5, 0.5),
            );
        }

        for selected in &self.selected {
            draw_circle(
                self.world_to_screen_coord(self.automaton.graph[*selected].position)
                    .x,
                self.world_to_screen_coord(self.automaton.graph[*selected].position)
                    .y,
                32.0 * self.zoom,
                Color::new(0.8, 0.8, 0.5, 1.0),
            )
        }
        for node in &self.automaton.graph.nodes {
            draw_circle(
                self.world_to_screen_coord(node.position).x,
                self.world_to_screen_coord(node.position).y,
                30.0 * self.zoom,
                if node.write {
                    Color::new(0.71, 0.643, 0.451, 1.0)
                } else {
                    Color::new(0.2, 0.7, 0.9, 1.0)
                },
            );
            draw_text(
                &node.ruleset,
                self.world_to_screen_coord(node.position).x - 30.0,
                self.world_to_screen_coord(node.position).y - 40.0 * self.zoom,
                18.0 * self.zoom,
                WHITE,
            );
        }

        for node in &self.automaton.graph.nodes {
            for connection in &node.edges {
                self.draw_arrow_world(
                    self.automaton.graph[*connection].position,
                    node.position,
                    30.0,
                )
            }
        }

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
    fn draw_arrow_world(&self, pos1: Vector2, pos2: Vector2, radius: f32) {
        let screen_pos2 = self.world_to_screen_coord(pos2);

        // Calculate the direction from pos1 to pos2
        let direction = (pos2 - pos1).normalized();

        // Calculate the start and end points on the circle
        let circle_start = pos1 + direction * radius;
        let circle_end = pos2 - direction * radius;

        let screen_circle_start = self.world_to_screen_coord(circle_start);
        let screen_circle_end = self.world_to_screen_coord(circle_end);

        // Draw the line from the edge of the circle to the arrowhead
        draw_line(
            screen_circle_start.x,
            screen_circle_start.y,
            screen_circle_end.x,
            screen_circle_end.y,
            2.0,
            WHITE,
        );

        // Calculate the angle of the arrow
        let angle =
            (screen_pos2.y - screen_circle_end.y).atan2(screen_pos2.x - screen_circle_end.x);

        // Calculate the points for the arrow triangle
        let arrow_length = 10.0 * self.zoom; // Length of the arrow's lines
        let arrow_width = 6.0 * self.zoom; // Width of the arrow's lines
        let arrow_offset = Vector2::new(arrow_length * angle.cos(), arrow_length * angle.sin());
        let arrow_point1 = screen_pos2 - arrow_offset
            + Vector2::new(arrow_width * angle.sin(), -arrow_width * angle.cos())
            - direction * radius * self.zoom;
        let arrow_point2 = screen_pos2
            - arrow_offset
            - Vector2::new(arrow_width * angle.sin(), -arrow_width * angle.cos())
            - direction * radius * self.zoom;

        // Draw the arrow triangle
        draw_triangle(
            (screen_pos2 - direction * radius * self.zoom).into(),
            arrow_point1.into(),
            arrow_point2.into(),
            WHITE,
        );
    }

    fn save_code(&self) {
        match rfd::FileDialog::new().save_file() {
            Some(file_path) => match File::create(file_path) {
                Ok(mut file) => {
                    if let Err(error) = file.write_all(self.code.as_bytes()) {
                        println!("unable to write to file: {}", error)
                    }
                }
                Err(err) => println!("unable to create file: {err}"),
            },
            _ => println!("no file chosen"),
        }
    }

    fn load_code(&mut self) {
        match rfd::FileDialog::new().pick_file() {
            Some(file_path) => match fs::read_to_string(file_path) {
                Ok(code) => self.code = code,
                Err(error) => println!("{error}"),
            },
            None => println!("no file picked"),
        }
    }

    fn save_graph(&self) {
        match rfd::FileDialog::new().save_file() {
            Some(file_path) => match File::create(file_path) {
                Ok(mut file) => {
                    if let Ok(serialized) = serde_json::to_string(&SavedState {
                        automaton: self.automaton.clone(),
                        code: self.code.clone(),
                    }) {
                        if let Err(error) = file.write_all(serialized.as_bytes()) {
                            println!("unable to write to file: {}", error)
                        }
                    }
                }
                Err(err) => println!("unable to create file: {err}"),
            },
            _ => println!("no file chosen"),
        }
    }

    fn load_graph(&mut self) {
        match rfd::FileDialog::new().pick_file() {
            Some(file_path) => match fs::read_to_string(file_path) {
                Ok(serialized) => match serde_json::from_str::<SavedState>(&serialized) {
                    Ok(state) => {
                        self.automaton = state.automaton;
                        self.code = state.code
                    }
                    Err(error) => println!("{error}"),
                },
                Err(error) => println!("{error}"),
            },
            None => println!("no file picked"),
        }
    }
}

fn find_rect(corner_1: Vector2, corner_2: Vector2) -> Rect {
    Rect {
        x: corner_1.x.min(corner_2.x),
        y: corner_1.y.min(corner_2.y),
        w: (corner_2.x - corner_1.x).abs(),
        h: (corner_2.y - corner_1.y).abs(),
    }
}
