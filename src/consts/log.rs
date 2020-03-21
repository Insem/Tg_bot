/*let timer = timer::Timer::new();
let (tx, rx) = channel();

let _guard = timer.schedule_with_delay(chrono::Duration::seconds(20), move || {
  // This closure is executed on the scheduler thread,
  // so we want to move it away asap.

  let _ignored = tx.send(()); // Avoid unwrapping here.
});
rx.recv().unwrap();
println!("This code has been executed after 3 seconds");
*/
//Так, пётр, ты пробовал все эти ебаные логгеры, остановился на ферн.
/*
fern::Dispatch::new()
.format(|out, message, record| {
    //настройка того как он будет выводить
    out.finish(format_args!(
        "{}[{}][{}] {}",
        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
        record.target(),
        record.level(),
        message
    ))
})
//слой инфо
.level(log::LevelFilter::Info)
.chain(std::io::stdout())
.chain(..важная тема, дрочит время к названию лог файла fern::DateBased::new("my-program.log.", "%S"))
.into_log();//Значит ебашить глобальный лог нельзя, тк он может быть только один
//поэтому ты решил, дрочить логи с помошью функции .into_log(), она создает файл
//но лог не дрочит
info!("xuuuut");//не работет

let (min_level, mylog) = fern::Dispatch::new()
.format(|out, message, record| {
out.finish(format_args!(
"{}[{}][{}] {}",
chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
record.target(),
record.level(),
message
))
})
.level(log::LevelFilter::Info)
.chain(std::io::stdout())
.chain(fern::DateBased::new("my-program_2.log.", "%S"))
.into_log();
mylog.log(&Record::builder()
//Здесь ты создаешь второй лог и создаешь в него рекорд. И пишешь трагет
.args(format_args!("Error!"))
.level(Level::Error)
.target("myApp")
.build());
info!(target:"myApp","rgerge")
//Здесь обращаешься к таргету, и эта хуйня работет
//Остается только это обернуть в ООП и ебнуть в интервал на каждый день
*/