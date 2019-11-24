///
//  Generated code. Do not modify.
//  source: protos/preferences.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class Preferences extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Preferences', createEmptyInstance: create)
    ..a<$core.int>(1, 'passphraseLength', $pb.PbFieldType.OU3, protoName: 'passphraseLength')
    ..aOB(2, 'addCapitalLetter', protoName: 'addCapitalLetter')
    ..aOB(3, 'addDigit', protoName: 'addDigit')
    ..aOB(4, 'addSymbol', protoName: 'addSymbol')
    ..hasRequiredFields = false
  ;

  Preferences._() : super();
  factory Preferences() => create();
  factory Preferences.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Preferences.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Preferences clone() => Preferences()..mergeFromMessage(this);
  Preferences copyWith(void Function(Preferences) updates) => super.copyWith((message) => updates(message as Preferences));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Preferences create() => Preferences._();
  Preferences createEmptyInstance() => create();
  static $pb.PbList<Preferences> createRepeated() => $pb.PbList<Preferences>();
  @$core.pragma('dart2js:noInline')
  static Preferences getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Preferences>(create);
  static Preferences _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get passphraseLength => $_getIZ(0);
  @$pb.TagNumber(1)
  set passphraseLength($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPassphraseLength() => $_has(0);
  @$pb.TagNumber(1)
  void clearPassphraseLength() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get addCapitalLetter => $_getBF(1);
  @$pb.TagNumber(2)
  set addCapitalLetter($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAddCapitalLetter() => $_has(1);
  @$pb.TagNumber(2)
  void clearAddCapitalLetter() => clearField(2);

  @$pb.TagNumber(3)
  $core.bool get addDigit => $_getBF(2);
  @$pb.TagNumber(3)
  set addDigit($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasAddDigit() => $_has(2);
  @$pb.TagNumber(3)
  void clearAddDigit() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get addSymbol => $_getBF(3);
  @$pb.TagNumber(4)
  set addSymbol($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasAddSymbol() => $_has(3);
  @$pb.TagNumber(4)
  void clearAddSymbol() => clearField(4);
}

