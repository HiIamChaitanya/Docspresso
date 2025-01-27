// src/main.qml
import QtQuick 2.15
import QtQuick.Controls 2.15

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: qsTr("My Qt App")

    Column {
        anchors.centerIn: parent
        spacing: 10

        Text {
            text: myObject.counter
            font.pixelSize: 24
        }

        Button {
            text: qsTr("Increment")
            onClicked: myObject.increment()
        }
    }
}
