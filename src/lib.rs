#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub path: std::path::PathBuf,
}

const PACKAGE_FILE_NAME: &'static str = "package.xml";
const ROS_PACKAGE_PATH_ENV_VAR: &'static str = "ROS_PACKAGE_PATH";

pub fn get_search_paths() -> Vec<std::path::PathBuf> {
    if let Ok(paths) = std::env::var(ROS_PACKAGE_PATH_ENV_VAR) {
        paths
            .split(std::path::MAIN_SEPARATOR)
            .into_iter()
            .map(|path| std::path::PathBuf::from(path))
            .collect::<Vec<std::path::PathBuf>>()
    } else {
        eprintln!("No ROS_PACKAGE_PATH defined.");
        vec![]
    }
}

pub fn crawl(search_paths: Vec<std::path::PathBuf>) -> Vec<Package> {
    let mut packages = vec![];

    for path in search_paths {
        if let Ok(found_packages) = packages_from_path(path) {
            packages = [packages, found_packages].concat();
        }
    }

    packages
}

fn packages_from_path(mut path: std::path::PathBuf) -> std::io::Result<Vec<Package>> {
    let mut found_packages = vec![];

    if path.as_path().is_dir() {
        // We have a valid path
        path.push(PACKAGE_FILE_NAME);
        if path.as_path().is_file() {
            // And there's a package.xml here!
            assert!(path.pop());
            found_packages.push(Package {
                name: String::from(path.file_name().unwrap().to_string_lossy()),
                path: path,
            })
        } else {
            // No file here, we'll have to go deeper
        }
    } else {
        eprintln!("{} is not a directory", path.to_string_lossy())
    }

    Ok(found_packages)
}
