use notify_rust::Notification;
use std::thread;     
use std::time::Duration;    

fn main() {
    let minutes = 10;
    let interval = Duration::from_secs(minutes * 60);

    println!("البرنامج بدأ يعمل في الخلفية.. سيتم تنبيهك كل {} دقيقة.", minutes);

    loop {
        thread::sleep(interval);

        Notification::new()
            .summary("تذكير شرب الماء 💧")
            .body("أيها المبرمج العظيم، حان وقت شرب الماء لتبقى نشيطاً!")
            .icon("dialog-information")   
            .timeout(5000)    
            .show()
            .unwrap();

        println!("تم إرسال التذكير!");
    }
}
