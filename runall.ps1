for ($i = 1; $i -le 25; $i++) {
    $name = 'day' + $i.ToString('00')
    cargo run --release -p $name
}