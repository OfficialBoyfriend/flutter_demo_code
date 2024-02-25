import 'package:flutter/widgets.dart';
import 'package:my_app/src/rust/api/mqtt.dart';

class MqttDemo extends StatefulWidget {
  const MqttDemo({super.key});

  @override
  MqttDemoState createState() => MqttDemoState();
}

class MqttDemoState extends State<MqttDemo> {
  late final Mqtt mqttClient;

  @override
  void initState() {
    super.initState();

    mqttClient = Mqtt.newMqtt(clientId: "clientId")
      ..connect(host: "host").listen((msg) {
        debugPrint("msg: $msg");
        mqttClient.send(payload: "success");
      });
  }

  @override
  Widget build(BuildContext context) {
    return const Text('MqttDemo');
  }
}
