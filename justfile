work day part:
  cargo watch -x "check -p {{day}}" -s "just test {{part}} -p {{day}}"
test day +FLAGS='':
  cargo nextest run {{day}} {{FLAGS}}
