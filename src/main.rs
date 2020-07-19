use rand::Rng;

// 年齢と名前を持った構造体
struct User {
  name: String,
  age: i32,
}

fn main() {
  // ユーザ一覧作成
  let num = 100;
  let mut rng = rand::thread_rng();
  let mut users: Vec<User> = Vec::with_capacity(num);
  for i in 0..num {
    users.push(
      User {
        name: i.to_string(),
        age: rng.gen_range(0, 120)
      }
    );
  }

  // バブルソート
  let size = users.len() - 1;
  for i in 0..size {
    for j in 0..(size - i) {
      if users[j].age > users[j + 1].age {
        users.swap(j, j + 1)
      }
    }
  }
  
  // 表示
  for user in &users {
    println!("No.{}: age {}", user.name, user.age);
  }
}