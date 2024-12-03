#include <Arduino.h>
#include <Servo.h>

// 핀맵 정의
const int digitalInputs[] = {2, 4, 7};
const int digitalOutputs[] = {8, 12, 13};
const int servoPins[] = {3, 5, 6, 9, 10, 11};
const int ledPin = A0;

// 서보 모터 객체 생성
Servo servos[6];

// LED 상태 변수
bool ledState = false;

// 마지막 LED 토글 시간
unsigned long lastToggle = 0;

// 설정 간격 (1초)
const unsigned long toggleInterval = 1000;

// 통신 프로토콜 상수
const char START_BIT = 'S'; // 시작 비트
const char END_BIT = 'E';   // 종료 비트
const char DELIMITER = ','; // 구분자

// 명령 처리 함수
void handleCommand(String cmd)
{
  if (cmd.charAt(0) != START_BIT || cmd.charAt(cmd.length() - 1) != END_BIT)
  {
    Serial.println("ERR:Invalid protocol format");
    return;
  }

  // 시작 비트와 종료 비트를 제외한 데이터 추출
  String data = cmd.substring(1, cmd.length() - 1);

  // 데이터를 쉼표로 분리
  int values[9]; // 6개 서보 + 3개 디지털 출력
  int index = 0;
  int lastIndex = 0;
  int pos = 0;

  while ((pos = data.indexOf(DELIMITER, lastIndex)) != -1 && index < 9)
  {
    values[index++] = data.substring(lastIndex, pos).toInt();
    lastIndex = pos + 1;
  }

  // 서보 모터 제어 (첫 6개 값)
  for (int i = 0; i < 6; i++)
  {
    if (values[i] >= 0 && values[i] <= 180)
    {
      servos[i].write(values[i]);
    }
  }

  // 디지털 출력 제어 (다음 3개 값)
  for (int i = 0; i < 3; i++)
  {
    digitalWrite(digitalOutputs[i], values[i + 6] ? HIGH : LOW);
  }

  // 현재 상태 응답
  String response = String(START_BIT);

  // 서보 각도 추가
  for (int i = 0; i < 6; i++)
  {
    response += String(servos[i].read());
    response += DELIMITER;
  }

  // 디지털 출력 상태 추가
  for (int i = 0; i < 3; i++)
  {
    response += String(digitalRead(digitalOutputs[i]));
    response += DELIMITER;
  }

  // 디지털 입력 상태 추가
  for (int i = 0; i < 3; i++)
  {
    response += String(digitalRead(digitalInputs[i]));
    if (i < 2)
      response += DELIMITER;
  }

  response += END_BIT;
  Serial.println(response);
}

// 초기화
void setup()
{
  Serial.begin(9600);
  while (!Serial)
  {
    ; // 시리얼 포트가 연결될 때까지 대기
  }

  // 연결 확인 메시지 전송
  Serial.println("Arduino Ready");

  // 디지털 입력 설정
  for (int pin : digitalInputs)
  {
    pinMode(pin, INPUT);
  }

  // 디지털 출력 설정
  for (int pin : digitalOutputs)
  {
    pinMode(pin, OUTPUT);
    digitalWrite(pin, LOW);
  }

  // LED 핀 설정
  pinMode(ledPin, OUTPUT);
  digitalWrite(ledPin, LOW);

  // 서보 모터 초기화
  for (int i = 0; i < 6; i++)
  {
    servos[i].attach(servoPins[i]);
    servos[i].write(90); // 초기 각도 90도
  }
}

void loop()
{
  if (Serial.available() > 0)
  {
    String command = Serial.readStringUntil('\n');
    command.trim();
    handleCommand(command);
  }

  // LED 상태 전환
  unsigned long currentMillis = millis();
  if (currentMillis - lastToggle >= toggleInterval)
  {
    ledState = !ledState;
    digitalWrite(ledPin, ledState ? HIGH : LOW);
    lastToggle = currentMillis;
  }
}
