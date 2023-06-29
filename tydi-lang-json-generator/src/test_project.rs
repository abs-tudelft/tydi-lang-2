use tydi_lang_parser::tydi_memory_representation::Project;

use crate::generate_json_representation_from_tydi_project;


#[test]
fn sample_project_rgb() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;

        bit_8 = Bit(8);
        bit_8_copy = Bit(8);

        Group rgb {
            r : bit_8;
            g : bit_8;
            b : bit_8_copy;
        }

        "#);
        let src_pack1 = format!("
        package pack1;
        use pack0;

        stream_rgb = Stream(pack0.rgb, d=2, u=pack0.bit_8);
        ");


        let status = project_write.add_package(format!("./pack0.td"), src_pack0);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print());
        }
        let status = project_write.add_package(format!("./pack1.td"), src_pack1);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print());
        }
    }
    project.read().unwrap().evaluate_target(format!("stream_rgb"), format!("pack1")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("stream_rgb"), format!("pack1")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn sample_project_union_0() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;

        bit_4 = Bit(4);
        bit_8 = Bit(8);
        bit_16 = Bit(16);

        Union size {
            small : bit_4;
            mid : bit_8;
            large : bit_16;
        }

        "#);
        let src_pack1 = format!("
        package pack1;
        use pack0;

        stream_size = Stream(pack0.size);
        ");


        let status = project_write.add_package(format!("./pack0.td"), src_pack0);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print());
        }
        let status = project_write.add_package(format!("./pack1.td"), src_pack1);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print());
        }
    }
    project.read().unwrap().evaluate_target(format!("stream_size"), format!("pack1")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("stream_size"), format!("pack1")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn sample_project_streamlet_0() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;

        bit_4 = Bit(4);
        bit_8 = Bit(8);
        bit_16 = Bit(16);

        Union size {
            small : bit_4;
            mid : bit_8;
            large : bit_16;
        }

        stream_size = Stream(size);

        "#);
        let src_pack1 = String::from(r#"
        package pack1;
        use pack0;

        streamlet bypass_s {
            port_in: pack0.stream_size in;
            port_out: pack0.stream_size out;
        }

        "#);

        let status = project_write.add_package(format!("./pack0.td"), src_pack0);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print());
        }
        let status = project_write.add_package(format!("./pack1.td"), src_pack1);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print());
        }
    }
    project.read().unwrap().evaluate_target(format!("bypass_s"), format!("pack1")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("bypass_s"), format!("pack1")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}