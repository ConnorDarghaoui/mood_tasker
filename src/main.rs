// Importar las dependencias
use reqwest::blocking::{Client, Response};
use scraper::{Html, Selector};
use std::fs::{create_dir_all, File};
use std::io::{stdin, stdout, Write};
use zip::ZipArchive;


// Definir la función input
fn input(prompt: &str) -> String {
    // Mostrar el mensaje al usuario
    print!("{}", prompt);
    // Vaciar el búfer estándar
    stdout().flush().unwrap();
    // Crear una variable para guardar la entrada
    let mut input = String::new();
    // Leer una línea desde el teclado
    stdin().read_line(&mut input).unwrap();
    // Eliminar el salto de línea final
    input.pop();
    // Devolver la entrada
    input
}

// Definir la función download_course
fn download_course(username: &str, password: &str, url: &str) -> (String, Vec<String>) {
    // Crear un cliente HTTP con cookies
    let client = Client::builder().cookie_store(true).build().unwrap();
    // Hacer una petición GET a la URL del curso
    let response = client.get(url).send().unwrap();
    // Comprobar si hay algún error
    if !response.status().is_success() {
        panic!("Error al acceder al curso");
    }
    // Obtener el HTML de la respuesta
    let html = response.text().unwrap();
    // Analizar el HTML con scraper
    let document = Html::parse_document(&html);
    // Crear un selector para extraer el nombre del curso
    let course_name_selector = Selector::parse("h1.page-header").unwrap();
    // Buscar el primer elemento que coincida con el selector
    let course_name_element = document.select(&course_name_selector).next().unwrap();
    // Obtener el texto del elemento
    let course_name_text = course_name_element.text().collect::<Vec<_>>();
    // Obtener la primera cadena del texto (sin espacios)
    let course_name = course_name_text[0].trim().to_string();
    // Crear un vector para guardar las secciones del curso
    let mut sections: Vec<String> = Vec::new();
    // Crear un selector para extraer los enlaces a las secciones
    let section_link_selector =
        Selector::parse("ul.topics li.section.main > div.content > h3.sectionname > a").unwrap();
    // Iterar sobre los elementos que coincidan con el selector
    for section_link_element in document.select(&section_link_selector) {
        // Obtener el atributo href del elemento (la URL de la sección)
        let section_link_value = section_link_element.value().attr("href").unwrap();
        // Añadir la URL al vector de secciones
        sections.push(section_link_value.to_string());
    }
    // Devolver una tupla con el nombre y las secciones del curso
    (course_name, sections)
}

// Definir la función unzip
fn unzip(zip_file: &str, destination: &str) {
    // Abrir el archivo ZIP como un objeto File
    let file = File::open(zip_file).unwrap();
    // Crear un objeto ZipArchive a partir del archivo ZIP
    let mut archive = ZipArchive::new(file).unwrap();
    // Iterar sobre los archivos dentro del ZIP
    for i in 0..archive.len() {
        // Obtener el archivo actual como un objeto ZipFile
        let mut file = archive.by_index(i).unwrap();
        // Obtener el nombre del archivo
        let name = file.name();
        // Crear un objeto File para escribir el contenido del archivo en el destino
        let mut output_file = File::create(format!("{}/{}", destination, name)).unwrap();
        // Escribir el contenido del archivo en el destino usando la biblioteca estándar de Rust
        std::io::copy(&mut file, &mut output_file).unwrap();
    }
}


fn main() {
    // Declarar las variables dentro de la función main
    let username: String;
    let password: String;
    let mut urls: Vec<String> = Vec::new();
    let mut courses: Vec<(String, Vec<String>)> = Vec::new();

    // Pedir al usuario su nombre de usuario y contraseña de Moodle
    username = input("Introduce tu nombre de usuario de Moodle: ");
    password = input("Introduce tu contraseña de Moodle: ");

    // Pedir al usuario las URL de los cursos de Moodle que quiere descargar
    println!("Introduce las URL de los cursos de Moodle que quieres descargar.");
    println!("Pulsa Enter sin escribir nada para terminar.");
    loop {
        // Pedir una URL al usuario
        let url = input("URL: ");
        // Comprobar si la URL está vacía
        if url.is_empty() {
            // Salir del bucle
            break;
        } else {
            // Añadir la URL al vector de URLs
            urls.push(url);
        }
    }

    // Iterar sobre el vector de URLs
    for url in &urls {
        // Descargar el curso correspondiente a la URL
        let course = download_course(&username, &password, &url);
        // Añadir el curso al vector de cursos
        courses.push(course);
    }

    // Iterar sobre el vector de cursos
    for course in &courses {
        // Obtener el nombre y las secciones del curso
        let (name, sections) = course;
        // Crear una carpeta con el nombre del curso
        create_dir_all(name).unwrap();
        // Descomprimir el contenido del curso dentro de la carpeta
        unzip(&(name.to_owned() + ".zip"), name);
    }
}

