use notify_rust::Notification;
use notify_rust::Hint;
use notify_rust::Urgency::{
    Low,
    Normal,
    Critical,
};
use std::process::{Command, exit, Stdio};
use std::process::id;
use std::fs::File;
use std::net::{SocketAddr, TcpStream};
use std::io::stdout;

pub fn sucesso() -> Result<(), notify_rust::error::Error> {
  Notification::new()
      .summary("Apk update")
      .body("Os repositorios foram atualizados com sucesso")
      .icon("/home/mimomu/m00alpineupdate/icons/mintupdate-up-to-date-symbolic.svg")
      .show()?;
    Ok(())
}
//fn main() {
//println!("Hello, world!");
//}

pub fn bemvindo() -> Result<(), notify_rust::error::Error> {
	Notification::new()
    	.summary("Category:Apk")
    	.body("Sejá bemvindo! Obrigado por usar m00alpineupdate. Atenção. Primeiro vamos a atualizar os repositorios. Por favor, espere.")
    	.icon("/home/mimomu/m00alpineupdate/icons/alpine-linux.svg")
    	.appname("m00alpineupdate")
    	.hint(Hint::Category("Apk".to_owned()))
    	.hint(Hint::Resident(true)) // this is not supported by all implementations
		.hint(Hint::Urgency(Critical))
    	.timeout(0) // this however is
    	.show()?;//
    Ok(())
}
pub fn umoutrotest() -> Result<(), notify_rust::error::Error> {
	Notification::new().summary("click me")
                   .action("default", "default")
                   .action("clicked", "click here")
                   .hint(Hint::Resident(true))
                   .show()
                   .unwrap()
                   .wait_for_action(|action| match action {
                                        "default" => println!("you clicked \"default\""),
                                        "clicked" => println!("that was correct"),
                                        // here "__closed" is a hard coded keyword
                                        "__closed" => println!("the notification was closed"),
                                        _ => ()
                                    });
    Ok(())
}
pub fn atualizarrepos() {
    let child = Command::new("sudo")
        .arg("apk")
        .arg("update")
        //.stdout() //Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    assert!(output.status.success());
    println!("O indice dos repositorios foram atualizados com sucesso");
    sucesso();
}

pub fn upgrade() {
    let child = Command::new("sudo")
        .arg("apk")
        .arg("upgrade")
        .arg("-vvs")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
        //.expect("failed to execute child");
	let linhas = Command::new("wc")
    	.arg("-l")
    	.stdin(Stdio::from(child.stdout.unwrap()))
    	.stdout(Stdio::piped())
    	.spawn()
    	.unwrap();
	/*let apks = Command::new("echo")
    	.arg("-")
    	.arg("1")
    	.stdin(Stdio::from(linhas.stdout.unwrap()))
    	.stdout(Stdio::piped())
    	.spawn()
    	.unwrap();*/
	//let dif = 1;

    /*let pacotes = Command::new("bc")
        .stdin(Stdio::from(linhas.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();*/
    let output = linhas
        .wait_with_output()
        .expect("failed to wait on child");

	//let apks = output - dif;
    //println!(" com sucessoPacotes igual: {}", apks);
	println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    assert!(output.status.success());
	//let int = output.to_string().
    //let output = output.parse::<i32>().unwrap();
    println!("Você tem {:#?} pacotes desatualizados", (&output.stdout));
    sucesso();

}

fn main() {
	let addrs = [
    	//SocketAddr::from(([127, 0, 0, 1], 8080)),
    	SocketAddr::from(([213,219,36,190], 80)),
	];
	if let Ok(stream) = TcpStream::connect(&addrs[..]) {
    	println!("Connected to the server!");
    	bemvindo();
    	atualizarrepos();
    	upgrade();
	} else {
    	println!("Não ha internet :(");
    	exit(1)
	};
    //let log_name = format!("./tmp/log/{}.log", id);
    //let log = File::create(log_name).expect("failed to open log");
//
    //let mut cmd = Command::new("echo")
        //.args(&[id])
        //.stdout(log)
        //.spawn()
        //.expect("failed to start echo");
//
    //cmd.wait().expect("failed to finish echo");

    //let mut child = Command::new("ls").spawn().unwrap();
//
	//match child.try_wait() {
    	//Ok(Some(status)) => println!("exited with: {status}"),
    	//Ok(None) => {
        	//println!("status not ready yet, let's really wait");
        	//let res = child.wait();
        	//println!("result: {res:?}");
    	//}
    	//Err(e) => println!("error attempting to wait: {e}"),
	//}

}
