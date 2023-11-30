use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;

const WORK_DIR : &str = "./lock_platform";
const URL_REPO : &str = "git@github.com:JonathanLT/lock_platform.git";

struct LockPlatform {
    url: String,
    path: String

}

impl LockPlatform {
    fn init(&self) -> bool {
        let _ = Command::new("git")
            .arg("clone")
            .arg(&self.url)
            .output()
            .expect("clone command failed");
        true
    }

    fn check(&self) -> bool {
        if env::set_current_dir(&self.path).is_ok() {
            let output = Command::new("git")
                .arg("--no-pager")
                .arg("log")
                .arg("-2")
                .arg("--pretty=format:'%an%x09%ad'")
                .arg("--date-order")
                .spawn()
                .expect("log command failed to start");
            println!("{:#?}", output.stdout);
            return env::set_current_dir(&self.path).is_ok()
        }
        false
    }

    fn lock(&self, pf:u8) -> bool {
        if env::set_current_dir(&self.path).is_ok() {
            let _ = Command::new("touch")
                .arg(format!("pf_{}", pf))
                .output()
                .expect("touch command failed");
            let _ = Command::new("git")
                .arg("add")
                .arg(format!("pf_{}", pf))
                .output()
                .expect("add command failed");
            let _ = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(format!("Lock pf_{}", pf))
                .output()
                .expect("commit command failed");
            let _ = Command::new("git")
                .arg("push")
                .output()
                .expect("commit command failed");
            return env::set_current_dir("..").is_ok()
        }
        false
    }

    fn unlock(&self, pf:u8) -> bool {
        if env::set_current_dir(&self.path).is_ok() {
            let pf_file = format!("pf_{}", pf);
            if Path::new(&pf_file).exists() {
                let _ = Command::new("git")
                    .arg("rm")
                    .arg(format!("pf_{}", pf))
                    .output()
                    .expect("rm command failed");
                let _ = Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg(format!("Unlock pf_{}", pf))
                    .output()
                    .expect("commit command failed");
                let _ = Command::new("git")
                    .arg("push")
                    .output()
                    .expect("commit command failed");
                return env::set_current_dir("..").is_ok()
            };
        }
        false
    }

    fn update(&self) -> bool {
        if env::set_current_dir(&self.path).is_ok() {
            let _ = Command::new("git")
                .arg("fetch")
                .arg("--all")
                .output()
                .expect("fetch command failed");
            let _ = Command::new("git")
                .arg("reset")
                .arg("--hard")
                .arg("HEAD")
                .output()
                .expect("reset command failed");
            let _ = Command::new("git")
                .arg("merge")
                .arg("@{u}")
                .output()
                .expect("merge command failed");
            return env::set_current_dir("..").is_ok()
        }
        false
    }

}


fn main() {
    if Path::new(&WORK_DIR).exists() {
        fs::remove_dir_all(WORK_DIR).unwrap();
    };
    let test: LockPlatform = LockPlatform { path: WORK_DIR.to_string(), url: URL_REPO.to_string() };

    test.init();
    test.lock(1);
    test.update();
    test.unlock(1);
    test.check();
}
