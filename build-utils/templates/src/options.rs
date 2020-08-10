use structopt::StructOpt;
use std::env;
use std::path::{Path, PathBuf};
use config::RemoteTarget;


#[derive(Debug, StructOpt)]
#[structopt(name = "database migrations", about = "A little util to run database migrations")]
pub struct Opts {
    // local, sandbox, or release 
    #[structopt(short, long)]
    pub remote_target: String,

    // project dir 
    #[structopt(short, long)]
    pub project: String,

    // show output 
    #[structopt(short, long)]
    pub verbose: bool,
}

impl Opts {
    pub fn get_remote_target(&self) -> RemoteTarget {

        match self.remote_target.as_ref() {  
            "local" => RemoteTarget::Local,
            "sandbox" => RemoteTarget::Sandbox,
            "release" => RemoteTarget::Release,
            _ => panic!("target must be local, sandbox, or release")
        }
    }

    pub fn get_project_template_path(&self) -> PathBuf {
        let path = self.get_project_path().join("templates");

        if !path.exists() {
            panic!("template path for [{}] does not exist!", &self.project);
        }

        path

    }
    pub fn get_core_template_path(&self) -> PathBuf {
        let path = self.get_frontend_path().join("_core").join("templates");

        if !path.exists() {
            panic!("core template path does not exist!");
        }

        path

    }

    pub fn get_project_output_path(&self) -> PathBuf {
        self.get_project_path().join(".template_output")
    }

    fn get_frontend_path(&self) -> PathBuf {
        Path::new(&env::var("LOCAL_CDN_FRONTEND_DIR").expect("needs env var LOCAL_CDN_FRONTEND_DIR")).to_path_buf()
    }

    fn get_project_path(&self) -> PathBuf {
        self.get_frontend_path()
            .join(&self.project)
    }
}


