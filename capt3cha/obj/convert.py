import sys
import os
import re


face_re = r"(?P<vert>\d+)/(?P<tex>\d*)/(?P<norm>\d+)"

def parse_line(line):
    # Find all matches in the line
    matches = re.finditer(face_re, line)
    
    # Initialize an empty list to store the results
    result = []
    
    # Iterate through the matches and extract the captured groups
    for match in matches:
        vert = int(match.group('vert'))
        norm = int(match.group('norm'))
        
        # Append a tuple of (vert, tex, norm) to the result list
        result.append((vert, None, norm))
    
    return result

def parse_obj_file(file_path):
    vertices = []
    normals = []
    texture_coords = []
    faces = []

    with open(file_path, "r") as file:
        for line in file:
            if line.startswith("#"):
                continue
            values = line.split()
            if not values:
                continue

            if values[0] == "v":
                v = map(float, values[1:4])
                vertices.append(list(v))
            elif values[0] == "vn":
                vn = map(float, values[1:4])
                normals.append(list(vn))
            elif values[0] == "vt":
                vt = map(float, values[1:4])
                texture_coords.append(list(vt))
            elif values[0] == "f":
                faces.append(parse_line(str(values[1:])))
    return vertices, normals, texture_coords, faces


def generate_rust_mesh(file_path, vertices, normals, texture_coords, faces):
    v_components = 3
    n_components = 3
    uv_components = 2
    draw_type = "gl::TRIANGLES"

    normals_idx = []
    for face in faces:
        normals_idx.append(face[0][2]-1)
        normals_idx.append(face[1][2]-1)
        normals_idx.append(face[2][2]-1)
    normals_f = [i for it in [normals[idx] for idx in normals_idx] for i in it]

    glyph_name = os.path.basename(file_path).split('.')[0]
    rust_code = f"""impl Mesh {{
    pub fn glyph_{glyph_name}() -> Mesh {{
        Mesh {{
            v_components: {v_components},
            vertices: vec![{','.join(map(str, [f'{v[0]:.6f}, {v[1]:.6f}, {v[2]:.6f}' for v in vertices]))}],
            indices: Some(vec![{','.join([str(i-1) for it in [[f[0][0], f[1][0], f[2][0]] for f in faces] for i in it])}]),
            n_components: {n_components},
            normals: Some(vec![{','.join([str(v) for v in normals_f])}]),
            uv_components: {uv_components},
            uv: Some(vec![{','.join(map(str, [f'{t[0]:.6f}, {t[1]:.6f}' for t in texture_coords]))}]),
            vbo_vertices: None,
            vbo_indices: None,
            vbo_normals: None,
            vbo_uv: None,
            vao: None,
            draw_type: {draw_type},
        }}
    }}
}}\n"""

    return rust_code


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python script.py <obj_file_path> <output path | default out.rs>")
        sys.exit(1)
    obj_file_path = sys.argv[1]
    try:
        vertices, normals, texture_coords, faces = parse_obj_file(obj_file_path)
        rust_mesh = generate_rust_mesh(
            obj_file_path, vertices, normals, texture_coords, faces
        )
        outpath = "out.rs"
        if len(sys.argv) == 3:
            outpath = sys.argv[2]
        with open(outpath, "a") as f:
            f.write(rust_mesh)
        print(f"Rust code generated successfully. Output file: {outpath}")
    except Exception as e:
        print(f"Error processing OBJ file: {str(e)}")
        raise e
