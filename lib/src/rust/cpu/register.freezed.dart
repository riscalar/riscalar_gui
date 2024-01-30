// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'register.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$XRegisterFile {
  U64Array32 get regs => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $XRegisterFileCopyWith<XRegisterFile> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $XRegisterFileCopyWith<$Res> {
  factory $XRegisterFileCopyWith(
          XRegisterFile value, $Res Function(XRegisterFile) then) =
      _$XRegisterFileCopyWithImpl<$Res, XRegisterFile>;
  @useResult
  $Res call({U64Array32 regs});
}

/// @nodoc
class _$XRegisterFileCopyWithImpl<$Res, $Val extends XRegisterFile>
    implements $XRegisterFileCopyWith<$Res> {
  _$XRegisterFileCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? regs = null,
  }) {
    return _then(_value.copyWith(
      regs: null == regs
          ? _value.regs
          : regs // ignore: cast_nullable_to_non_nullable
              as U64Array32,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$XRegisterFileImplCopyWith<$Res>
    implements $XRegisterFileCopyWith<$Res> {
  factory _$$XRegisterFileImplCopyWith(
          _$XRegisterFileImpl value, $Res Function(_$XRegisterFileImpl) then) =
      __$$XRegisterFileImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({U64Array32 regs});
}

/// @nodoc
class __$$XRegisterFileImplCopyWithImpl<$Res>
    extends _$XRegisterFileCopyWithImpl<$Res, _$XRegisterFileImpl>
    implements _$$XRegisterFileImplCopyWith<$Res> {
  __$$XRegisterFileImplCopyWithImpl(
      _$XRegisterFileImpl _value, $Res Function(_$XRegisterFileImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? regs = null,
  }) {
    return _then(_$XRegisterFileImpl(
      regs: null == regs
          ? _value.regs
          : regs // ignore: cast_nullable_to_non_nullable
              as U64Array32,
    ));
  }
}

/// @nodoc

class _$XRegisterFileImpl implements _XRegisterFile {
  const _$XRegisterFileImpl({required this.regs});

  @override
  final U64Array32 regs;

  @override
  String toString() {
    return 'XRegisterFile(regs: $regs)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$XRegisterFileImpl &&
            const DeepCollectionEquality().equals(other.regs, regs));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(regs));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$XRegisterFileImplCopyWith<_$XRegisterFileImpl> get copyWith =>
      __$$XRegisterFileImplCopyWithImpl<_$XRegisterFileImpl>(this, _$identity);
}

abstract class _XRegisterFile implements XRegisterFile {
  const factory _XRegisterFile({required final U64Array32 regs}) =
      _$XRegisterFileImpl;

  @override
  U64Array32 get regs;
  @override
  @JsonKey(ignore: true)
  _$$XRegisterFileImplCopyWith<_$XRegisterFileImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
