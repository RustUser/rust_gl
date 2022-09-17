use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::ops::Index;
use std::path::Path;
use enum_iterator::{all, Sequence};
use maplit::hashmap;
use crate::math::linear_algebra::types::{Vec2, Vec3};
use crate::utils::data_structure::DataStructure;
use crate::{BufferDataType, BufferType, Constructor, DrawType, LocalAttribPointer, VertexArrayObject, VertexArrayObjectType, VertexBufferObject};

pub type Vertex = (usize, Option<usize>, Option<usize>);
pub type Face = [Vertex; 3];

#[derive(Clone)]
pub enum OBJError {
    MaterialLibraryCount(String, usize),
    VertexSize(String, usize),
    VertexTextureSize(String, usize),
    VertexNormalSize(String, usize),
    VertexRect(String, usize),
    ObjectNotCreated(String, usize),
    MaterialUsage(String, usize),
    FaceLength(String, usize),
    NoGroupFound(String, usize),
    ShadingOption(String, usize),
}

impl Debug for OBJError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OBJError::MaterialLibraryCount(error, ln) => {
                f.write_fmt(format_args!("Material libraries can only have one path: {} on line {}", error, ln))
            }
            OBJError::VertexSize(vertex, ln) => {
                f.write_fmt(format_args!("Vertex size is out of bounds. Must be 3/4: {} on line {}", vertex, ln))
            }
            OBJError::VertexRect(rect, ln) => {
                f.write_fmt(format_args!("Rects are not supported: {} on line {}", rect, ln))
            }
            OBJError::ObjectNotCreated(e, ln) => {
                f.write_fmt(format_args!("Unable to perform obj command because no object had been selected: {} on line {}", e, ln))
            }
            OBJError::VertexTextureSize(tex_coord, ln) => {
                f.write_fmt(format_args!("Vertex texture coord size is out of bounds. Must be 2/3: {} on line {}", tex_coord, ln))
            }
            OBJError::VertexNormalSize(normal_size, ln) => {
                f.write_fmt(format_args!("Vertex normal size is out of bounds. Must be 3/4: {} on line {}", normal_size, ln))
            }
            OBJError::MaterialUsage(usage, ln) => {
                f.write_fmt(format_args!("Too many materials defined. Only one can be defined: {} on line {}", usage, ln))
            }
            OBJError::FaceLength(face, ln) => {
                f.write_fmt(format_args!("Must supply face in groups of 3. {} on line {}", face, ln))
            }
            OBJError::NoGroupFound(group, ln) => {
                f.write_fmt(format_args!("Unable to insert face data in null group. {} on line {}", group, ln))
            }
            OBJError::ShadingOption(shading, ln) => {
                f.write_fmt(format_args!("Shading option not supported: {} on line {}", shading, ln))
            }
        }
    }
}

impl Display for OBJError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for OBJError {}

#[derive(Debug, Clone, Sequence)]
///Wavefont obj object command. See: <a href="https://en.wikipedia.org/wiki/Wavefront_.obj_file#File_format">OBJ</a>
pub enum OBJCommand {
    ///# this is a comment
    Comment,
    ///List of geometric vertices, with (x, y, z, [w]) coordinates, w is optional and defaults to 1.0.
    Vertex,

    ///Texture coordinates, in (u, [v, w]) coordinates, these will vary between 0 and 1. v, w are optional and default to 0.
    VertexTextureCoordinate,

    ///Vertex normals in (x,y,z) form; normals might not be unit vectors.
    VertexNormal,

    ///Parameter space vertices in (u, [v, w]) form; free form geometry statement (see below); ignored
    ParameterSpaceVertex,

    ///Polygonal face data. 1-indexed. Vertex/Vertex Texture Coordinate/Vertex Normal
    Face,

    ///Line element. 1-indexed. Example: 'l 5 8 1 2 4 9'
    Line,

    UseMaterial,

    MaterialLibrary,

    Object,

    Smooth,
}

impl OBJCommand {
    fn vertex_from_raw(raw: &str) -> Result<Vertex, Box<ParseIntError>> {
        let data = raw.split("/").collect::<Vec<&str>>();

        let v = data[0].parse::<usize>()? - 1;
        let vt = match data.get(1) {
            None => {
                None
            }
            Some(vt) => {
                Some(vt.parse::<usize>()? - 1)
            }
        };
        let vn = match data.get(2) {
            None => {
                None
            }
            Some(vn) => {
                Some(vn.parse::<usize>()? - 1)
            }
        };
        Ok((v, vt, vn))
    }
    pub fn process_input(&self, line_number: usize, command: &str, input: &[&str], material_libraries: &mut Vec<String>, current_object: &mut Option<Object>, object_buffer: &mut Vec<Object>) -> Result<(), Box<dyn Error>> {
        match self {
            OBJCommand::Vertex => {
                if input.len() == 4 {
                    return Err(Box::new(OBJError::VertexSize(input.join(" "), line_number)));
                }
                match input.len() == 3 {
                    true => {
                        match current_object {
                            None => {
                                return Err(Box::new(OBJError::ObjectNotCreated(format!("v {}", input.join(" ")), line_number)));
                            }
                            Some(obj) => {
                                let x = input[0].parse::<f32>()?;
                                let y = input[1].parse::<f32>()?;
                                let z = input[2].parse::<f32>()?;
                                obj.vertices.push([x, y, z]);
                            }
                        }
                    }
                    false => {
                        return Err(Box::new(OBJError::VertexSize(input.join(" "), line_number)));
                    }
                }
            }
            OBJCommand::VertexTextureCoordinate => {
                if input.len() > 2 {
                    return Err(Box::new(OBJError::VertexTextureSize(input.join(" "), line_number)));
                }
                match input.len() == 2 {
                    true => {
                        match current_object {
                            None => {
                                return Err(Box::new(OBJError::ObjectNotCreated(format!("vt {}", input.join(" ")), line_number)));
                            }
                            Some(obj) => {
                                let u = input[0].parse::<f32>()?;
                                let v = input[1].parse::<f32>()?;
                                obj.uvs.push([u, v]);
                            }
                        }
                    }
                    false => {
                        return Err(Box::new(OBJError::VertexSize(input.join(" "), line_number)));
                    }
                }
            }
            OBJCommand::VertexNormal => {
                match input.len() == 3 {
                    true => {
                        match current_object {
                            None => {
                                return Err(Box::new(OBJError::ObjectNotCreated(format!("vn {}", input.join(" ")), line_number)));
                            }
                            Some(obj) => {
                                let x = input[0].parse::<f32>()?;
                                let y = input[1].parse::<f32>()?;
                                let z = input[2].parse::<f32>()?;
                                obj.normals.push([x, y, z]);
                            }
                        }
                    }
                    false => {
                        return Err(Box::new(OBJError::VertexNormalSize(input.join(" "), line_number)));
                    }
                }
            }
            OBJCommand::Face => {
                if input.len() == 3 {
                    if let Some(current_object) = current_object {
                        if let Some(group) = current_object.groups.last_mut() {
                            let faces = &mut group.faces;

                            let a = Self::vertex_from_raw(input[0])?;
                            let b = Self::vertex_from_raw(input[1])?;
                            let c = Self::vertex_from_raw(input[2])?;
                            let face = [a, b, c];
                            faces.push(face);
                        } else {
                            return Err(Box::new(OBJError::NoGroupFound(input.join(" "), line_number)));
                        }
                    } else {
                        return Err(Box::new(OBJError::ObjectNotCreated(input.join(" "), line_number)));
                    }
                } else {
                    return Err(Box::new(OBJError::FaceLength(input.join(" "), line_number)));
                }
            }
            OBJCommand::UseMaterial => {
                if input.len() > 1 {
                    return Err(Box::new(OBJError::MaterialUsage(input.join(" "), line_number)));
                }
                //Create new group.
                match current_object {
                    None => {
                        return Err(Box::new(OBJError::ObjectNotCreated("Cannot use a material if no object is currently selected.".to_string(), line_number)));
                    }
                    Some(current_object) => {
                        current_object.groups.push(Group {
                            material: input[0].to_string(),
                            faces: vec![],
                            shading: false,
                        });
                    }
                }
            }
            OBJCommand::MaterialLibrary => {
                //Example: mtllib cube.mtl
                match input.len() == 1 {
                    true => {
                        material_libraries.push(input[0].to_string());
                    }
                    false => {
                        return Err(Box::new(OBJError::MaterialLibraryCount(input.join(" "), line_number)));
                    }
                }
            }
            OBJCommand::Object => {
                match input.len() == 1 {
                    true => {
                        if let Some(current_object) = current_object {
                            object_buffer.push(current_object.clone());
                        }
                        *current_object = Some(Object {
                            name: input[0].to_string(),
                            vertices: vec![],
                            normals: vec![],
                            uvs: vec![],
                            groups: vec![],
                            material_libraries: material_libraries.clone(),
                        });
                        material_libraries.clear();
                    }
                    false => {
                        return Err(Box::new(OBJError::MaterialLibraryCount(input.join(" "), line_number)));
                    }
                }
            }
            OBJCommand::Smooth => {
                if let Some(current_object) = current_object {
                    if let Some(group) = current_object.groups.last_mut() {
                        if input[0] == "on" {
                            group.shading = true;
                        } else if input[0] == "off" {
                            group.shading = false;
                        } else {
                            return Err(Box::new(OBJError::ShadingOption(input.join(" "), line_number)));
                        }
                    } else {
                        return Err(Box::new(OBJError::NoGroupFound(input.join(" "), line_number)));
                    }
                } else {
                    return Err(Box::new(OBJError::ObjectNotCreated(input.join(" "), line_number)));
                }
            }
            _ => {
                println!("Command {} has been ignored.", command);
            }
        }
        Ok(())
    }
}

impl PartialEq<&str> for OBJCommand {
    fn eq(&self, other: &&str) -> bool {
        match self {
            OBJCommand::Comment => {
                if *other == "#" {
                    return true;
                }
            }
            OBJCommand::Vertex => {
                if *other == "v" {
                    return true;
                }
            }
            OBJCommand::VertexTextureCoordinate => {
                if *other == "vt" {
                    return true;
                }
            }
            OBJCommand::VertexNormal => {
                if *other == "vn" {
                    return true;
                }
            }
            OBJCommand::ParameterSpaceVertex => {
                if *other == "vp" {
                    return true;
                }
            }
            OBJCommand::Face => {
                if *other == "f" {
                    return true;
                }
            }
            OBJCommand::Line => {
                if *other == "l" {
                    return true;
                }
            }
            OBJCommand::MaterialLibrary => {
                if *other == "mtllib" {
                    return true;
                }
            }
            OBJCommand::Object => {
                if *other == "o" {
                    return true;
                }
            }
            OBJCommand::Smooth => {
                if *other == "s" {
                    return true;
                }
            }
            OBJCommand::UseMaterial => {
                if *other == "usemtl" {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    name: String,
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
    groups: Vec<Group>,
    material_libraries: Vec<String>,
}

impl<T: ToString> Index<T> for Object {
    type Output = Group;

    fn index(&self, index: T) -> &Self::Output {
        let index = index.to_string();
        for group in &self.groups {
            if group.material == index {
                return group;
            }
        }
        panic!("Could not find group")
    }
}

impl Object {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }
    pub fn normals(&self) -> &Vec<Vec3> {
        &self.normals
    }
    pub fn uvs(&self) -> &Vec<Vec2> {
        &self.uvs
    }
    pub fn groups(&self) -> &Vec<Group> {
        &self.groups
    }
    pub fn material_libraries(&self) -> &Vec<String> {
        &self.material_libraries
    }

    fn put_positions(out_positions: &mut Vec<f32>, vertex: &Vertex, vertices: &Vec<Vec3>, uvs: &&Vec<Vec2>, normals: &&Vec<Vec3>) {
        let position = vertices[vertex.0];
        out_positions.push_array(&position);
        if let Some(normal) = vertex.2 {
            let normal = normals[normal];
            out_positions.push_array(&normal);
        }
        if let Some(uv) = vertex.1 {
            let uv = uvs[uv];
            out_positions.push_array(&uv);
        }
    }

    pub fn build_vaos(&self) -> HashMap<String, Option<VertexArrayObject>> {
        let mut vaos = hashmap! {};
        for g in &self.groups {
            vaos.insert(g.material.clone(), self.build_vao(g.material.clone()));
        }
        vaos
    }

    pub fn build_vao<T: ToString>(&self, material: T) -> Option<VertexArrayObject> {
        let group = material.to_string();
        for g in &self.groups {
            if g.material.eq(&group) {
                let mut positions = vec![];
                let vertices = &self.vertices;
                let uvs = &self.uvs;
                let normals = &self.normals;

                for face in &g.faces {
                    for vertex in face {
                        Self::put_positions(&mut positions, vertex, vertices, &uvs, &normals);
                    }
                }

                let mut pointers = vec![
                    LocalAttribPointer::new(3, BufferDataType::Float, false)
                ];
                if g.has_normals() {
                    pointers.push(
                        LocalAttribPointer::new(3, BufferDataType::Float, false)
                    );
                }
                if g.has_uvs() {
                    pointers.push(
                        LocalAttribPointer::new(2, BufferDataType::Float, false)
                    );
                }
                return Some(
                    VertexArrayObject::new(Some(VertexArrayObjectType::Arrays((g.faces.len() * 3) as i32)))
                        .with_buffer(VertexBufferObject::array(BufferType::ArrayBuffer, DrawType::StaticDraw, &positions))
                        .with_local_attrib_pointers(pointers)
                        .build()
                )
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    material: String,
    faces: Vec<Face>,
    shading: bool,
}

impl Group {
    pub fn has_normals(&self) -> bool {
        if self.faces.len() == 0 {
            return false;
        }
        self.faces[0][0].2.is_some()
    }

    pub fn has_uvs(&self) -> bool {
        if self.faces.len() == 0 {
            return false;
        }
        self.faces[0][0].1.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct OBJ {
    objects: Vec<Object>,
}

impl OBJ {
    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<OBJ, Box<dyn Error>> {
        match std::fs::read_to_string(file) {
            Ok(contents) => {
                Self::from_raw(contents)
            }
            Err(e) => {
                Err(Box::new(e))
            }
        }
    }

    fn all_commands() -> Vec<OBJCommand> {
        let mut commands = vec![];
        let cmds = all::<OBJCommand>();
        for cmd in cmds {
            commands.push(cmd);
        }
        commands
    }

    pub fn from_raw(_raw: String) -> Result<OBJ, Box<dyn Error>> {
        let commands = Self::all_commands();
        let mut material_libraries: Vec<String> = vec![];
        let mut object_buffer = vec![];
        let mut current_object = None;

        let mut line_number = 1;
        for line in _raw.split("\n") {
            let line = line.trim();
            if line.is_empty() {
                line_number += 1;
                continue;
            }
            let contents = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let command = &contents[0];
            let contents = &contents[1..];

            let cmd = commands.iter().find(|cmd| *cmd == command).expect(format!("{} is not a valid obj command.", command).as_str());

            if let Err(e) = cmd.process_input(line_number, command, contents, &mut material_libraries, &mut current_object, &mut object_buffer) {
                return Err(e);
            }
            line_number += 1;
        }
        if let Some(current_object) = current_object {
            object_buffer.push(current_object.clone());
        }
        Ok(OBJ {
            objects: object_buffer
        })
    }

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }
}

impl<T: ToString> Index<T> for OBJ {
    type Output = Object;

    fn index(&self, index: T) -> &Self::Output {
        let index = index.to_string();
        for s in &self.objects {
            if s.name == index {
                return s;
            }
        }
        panic!("Could not find it")
    }
}