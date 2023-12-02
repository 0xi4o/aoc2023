work day part:
  cargo watch -x "check -p {{day}}" -s "just test {{part}} -p {{day}}"
dev day +FLAGS='':
  cargo watch -s "just test -p {{day}} {{FLAGS}}"
test day +FLAGS='':
  cargo nextest run {{day}} {{FLAGS}}
