use schemars::{
    gen::SchemaGenerator,
    schema::{Schema, SchemaObject},
    JsonSchema,
};

#[derive(JsonSchema)]
/// # UN/LOCODE Data Model Extension (Version 0.1.0-20230522)
///
/// The LOCODE data model extension. See https://sine-fdn.github.io/locode-extension for details
pub struct LoCodeExtension {
    /// The property `unlocode`. See https://sine-fdn.github.io/locode-extension/#element-attrdef-productfootprint-unlocode
    #[schemars(schema_with = "locode_schema")]
    pub unlocode: String,

    /// The property `coordinates`. See https://sine-fdn.github.io/locode-extension/#element-attrdef-productfootprint-coordinates
    #[schemars(schema_with = "coordinates_schema")]
    #[schemars(default)]
    pub coordinates: Option<String>,

    //pub coordinates: Option<Coordinates>,
    /// The property `name`. See https://sine-fdn.github.io/locode-extension/#element-attrdef-productfootprint-name
    pub name: Option<String>,

    /// The property `remarks`. See https://sine-fdn.github.io/locode-extension/#element-attrdef-productfootprint-remarks
    pub remarks: Option<String>,
}

fn locode_schema(gen: &mut SchemaGenerator) -> Schema {
    let mut s: SchemaObject = Option::<String>::json_schema(gen).into();

    s.metadata = Some(Box::new(schemars::schema::Metadata {
        description: Some("The UN/LOCODE is a geographic coding scheme developed and maintained by United Nations Economic Commission for Europe (UNECE).".into()),
        examples: vec![serde_json::Value::String("ZMCHZ".into())],
        ..Default::default()
    }));
    s.string = Some(Box::new(schemars::schema::StringValidation {
        max_length: Some(5),
        min_length: Some(5),
        pattern: Some(r"^[A-Z]{2}[A-Z2-9]{3}$".into()),
    }));

    Schema::Object(s)
}

fn coordinates_schema(gen: &mut SchemaGenerator) -> Schema {
    let mut s = match String::json_schema(gen) {
        Schema::Object(s) => s,
        Schema::Bool(_) => panic!("Unexpected base schema"),
    };

    s.metadata = Some(Box::new(schemars::schema::Metadata {
        description: Some("The coordinates of the location".into()),
        examples: vec![serde_json::Value::String("0917S 03220E".into())],
        ..Default::default()
    }));

    s.string = Some(Box::new(schemars::schema::StringValidation {
        max_length: Some(12),
        min_length: Some(12),
        pattern: Some(r"^[0-9]{4}[NS] [0-9]{5}[WE]$".into()),
    }));

    Schema::Object(s)
}

fn main() {
    //let schema = schema_for!(LoCodeExtension);
    let settings = schemars::gen::SchemaSettings::draft2019_09().with(|s| {
        s.option_nullable = true;
        s.option_add_null_type = false;
    });
    let gen = settings.into_generator();
    let mut schema = gen.into_root_schema_for::<LoCodeExtension>();
    schema.schema.metadata = schema.schema.metadata.map(|m| Box::new({schemars::schema::Metadata {
        id: Some("https://github.com/sine-fdn/locode-extension/blob/main/specs/locode_extension.json".into()),
        ..(*m)
    }}));
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
