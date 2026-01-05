// Copy vs Clone 차이점 설명

// ============================================
// 1. Copy 트레이트
// ============================================
// Copy는 "마커 트레이트"입니다 - 실제 메서드가 없습니다
// Copy를 구현하는 타입은 값이 이동(move)될 때 자동으로 복사됩니다

fn copy_example() {
    let x = 5;  // i32는 Copy 타입
    let y = x;  // x가 y로 이동하는 것이 아니라, x가 자동으로 복사됨
    
    println!("x = {}, y = {}", x, y);  // x도 여전히 사용 가능!
    
    // Copy 타입: i32, bool, char, f64, 배열 [T; N] (T가 Copy일 때)
    // Copy는 항상 저렴한 연산 (스택 복사만)
}

// ============================================
// 2. Clone 트레이트
// ============================================
// Clone은 명시적으로 호출해야 하는 메서드입니다
// .clone()을 호출해야 복사가 일어납니다

fn clone_example() {
    let s1 = String::from("hello");  // String은 Clone이지만 Copy는 아님
    let s2 = s1.clone();  // 명시적으로 clone() 호출 필요
    
    println!("s1 = {}, s2 = {}", s1, s2);  // 둘 다 사용 가능
    
    // 만약 let s2 = s1; 라고 하면?
    // s1은 s2로 이동되고, s1은 더 이상 사용할 수 없음!
}

// ============================================
// 3. Copy vs Clone 비교
// ============================================

fn comparison_example() {
    // Copy 타입 (i32)
    let a = 10;
    let b = a;  // 자동 복사 (암묵적)
    println!("a = {}, b = {}", a, b);  // 둘 다 사용 가능
    
    // Clone 타입 (String)
    let s1 = String::from("world");
    let s2 = s1.clone();  // 명시적 복사 필요
    println!("s1 = {}, s2 = {}", s1, s2);  // 둘 다 사용 가능
    
    // 만약 let s2 = s1; 라고 하면?
    // let s2 = s1;  // 이렇게 하면 s1이 이동됨
    // println!("{}", s1);  // 컴파일 에러! s1은 더 이상 사용 불가
}

// ============================================
// 4. 함수 파라미터로 전달할 때
// ============================================

fn take_ownership_copy(x: i32) {
    println!("{}", x);
}

fn take_ownership_clone(s: String) {
    println!("{}", s);
}

fn ownership_example() {
    let num = 5;
    take_ownership_copy(num);  // num이 복사되어 전달됨
    println!("num은 여전히 사용 가능: {}", num);  // ✅ OK
    
    let text = String::from("hello");
    take_ownership_clone(text);  // text가 이동됨
    // println!("{}", text);  // ❌ 컴파일 에러! text는 더 이상 사용 불가
    
    // Clone을 사용하려면:
    let text2 = String::from("world");
    take_ownership_clone(text2.clone());  // 명시적으로 복사
    println!("text2는 여전히 사용 가능: {}", text2);  // ✅ OK
}

// ============================================
// 5. Copy와 Clone의 관계
// ============================================
// Copy를 구현하는 타입은 반드시 Clone도 구현해야 합니다
// 하지만 Copy는 자동 복사, Clone은 명시적 복사

#[derive(Copy, Clone)]  // 둘 다 구현
struct Point {
    x: i32,
    y: i32,
}

// String은 Clone만 구현 (Copy는 아님)
// let s = String::from("test");
// s.clone()  // 명시적으로 호출해야 함

// ============================================
// 6. 언제 Copy를 사용할 수 있나?
// ============================================
// Copy는 다음 조건을 만족해야 합니다:
// 1. 타입의 모든 필드가 Copy 타입이어야 함
// 2. Drop 트레이트를 구현하지 않아야 함
// 3. 힙 할당이 없어야 함 (스택에만 존재)

// ✅ Copy 가능
#[derive(Copy, Clone)]
struct Simple {
    x: i32,
    y: bool,
}

// ❌ Copy 불가능 (String은 Copy가 아님)
// #[derive(Copy, Clone)]  // 에러!
struct Complex {
    name: String,  // String은 Copy가 아님
}

// ============================================
// 7. 성능 차이
// ============================================

fn performance_difference() {
    // Copy: 매우 저렴 (메모리 복사만)
    let a = 42;
    let b = a;  // 단순 메모리 복사 (나노초 단위)
    
    // Clone: 비용이 높을 수 있음 (힙 할당 가능)
    let s1 = String::from("very long string...");
    let s2 = s1.clone();  // 힙 메모리 할당 및 복사 (마이크로초~밀리초)
}

// ============================================
// 요약
// ============================================
// Copy:
// - 자동 복사 (암묵적)
// - 항상 저렴한 연산
// - 스택에만 존재하는 타입
// - 이동 대신 복사가 일어남
//
// Clone:
// - 명시적 복사 (.clone() 호출 필요)
// - 비용이 높을 수 있음
// - 힙 할당 가능
// - 이동을 방지하기 위해 사용




