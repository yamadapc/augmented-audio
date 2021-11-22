// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'audio_io_service.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RemoteAudioDevice _$RemoteAudioDeviceFromJson(Map<String, dynamic> json) =>
    RemoteAudioDevice(
      name: json['name'] as String,
      sampleRateRange: (json['sampleRateRange'] as List<dynamic>)
          .map((e) => e as int)
          .toList(),
      bufferSizeRange: (json['bufferSizeRange'] as List<dynamic>?)
          ?.map((e) => e as int)
          .toList(),
    );

Map<String, dynamic> _$RemoteAudioDeviceToJson(RemoteAudioDevice instance) =>
    <String, dynamic>{
      'name': instance.name,
      'sampleRateRange': instance.sampleRateRange,
      'bufferSizeRange': instance.bufferSizeRange,
    };

RemoteDevicesList _$RemoteDevicesListFromJson(Map<String, dynamic> json) =>
    RemoteDevicesList(
      inputDevices: (json['inputDevices'] as List<dynamic>)
          .map((e) => RemoteAudioDevice.fromJson(e as Map<String, dynamic>))
          .toList(),
      outputDevices: (json['outputDevices'] as List<dynamic>)
          .map((e) => RemoteAudioDevice.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$RemoteDevicesListToJson(RemoteDevicesList instance) =>
    <String, dynamic>{
      'inputDevices': instance.inputDevices,
      'outputDevices': instance.outputDevices,
    };
