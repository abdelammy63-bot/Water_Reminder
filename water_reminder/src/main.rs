use notify_rust::Notification; // مكتبة الإشعارات
use std::thread; // للتعامل مع الوقت والخلفية
use std::time::Duration; // لتحديد المدة الزمنية

fn main() {
    // تحديد عدد الدقائق بين كل تنبيه (مثلاً كل 30 دقيقة)
    let minutes = 10;
    let interval = Duration::from_secs(minutes * 60);

    println!("البرنامج بدأ يعمل في الخلفية.. سيتم تنبيهك كل {} دقيقة.", minutes);

    loop {
        // اجعل البرنامج "ينام" للمدة المحددة
        thread::sleep(interval);

        // إرسال إشعار لسطح المكتب
        Notification::new()
            .summary("تذكير شرب الماء 💧")
            .body("أيها المبرمج العظيم، حان وقت شرب الماء لتبقى نشيطاً!")
            .icon("dialog-information") // أيقونة افتراضية
            .timeout(5000) // يختفي بعد 5 ثواني
            .show()
            .unwrap();

        println!("تم إرسال التذكير!");
    }
}
