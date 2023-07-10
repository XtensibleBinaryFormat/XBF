#pragma once

#include <boost/archive/text_oarchive.hpp>
#include <boost/archive/text_iarchive.hpp>
#include <boost/serialization/vector.hpp>
#include <boost/serialization/map.hpp>
#include <boost/serialization/base_object.hpp>
#include <boost/serialization/export.hpp>

enum class PrimitiveType {
    Boolean,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    I8,
    I16,
    I32,
    I64,
    I128,
    I256,
    F32,
    F64,
    Bytes,
    String
};

class Metadata {
public:
    virtual ~Metadata() {}
    virtual PrimitiveType getType() const = 0; // Pure virtual function

private:
    friend class boost::serialization::access;

    template <class Archive>
    void serialize(Archive& ar, const unsigned int version) {}
};

class PrimitiveMetadata : public Metadata {
public:
    PrimitiveMetadata() {}
    PrimitiveMetadata(PrimitiveType type) : type_(type) {}

    PrimitiveType getType() const override {
        return type_;
    }

private:
    friend class boost::serialization::access;

    template <class Archive>
    void serialize(Archive& ar, const unsigned int version) {
        ar & boost::serialization::base_object<Metadata>(*this);
        ar & type_;
    }

    PrimitiveType type_;
};

class VecMetadata : public Metadata {
public:
    VecMetadata() {}
    VecMetadata(const Metadata* internalType) : internalType_(internalType) {}

    const Metadata* getInternalType() const {
        return internalType_;
    }

    PrimitiveType getType() const override {
        return PrimitiveType::U32; // Return the type of VecMetadata
    }

private:
    friend class boost::serialization::access;

    template <class Archive>
    void serialize(Archive& ar, const unsigned int version) {
        ar & boost::serialization::base_object<Metadata>(*this);
        ar & internalType_;
    }

    const Metadata* internalType_;
};

class StructMetadata : public Metadata {
public:
    StructMetadata() {}

    void addField(const std::string& name, const Metadata* fieldType) {
        fields_[name] = fieldType;
    }

    const std::map<std::string, const Metadata*>& getFields() const {
        return fields_;
    }

    PrimitiveType getType() const override {
        return PrimitiveType::U8; // Return the type of StructMetadata
    }

private:
    friend class boost::serialization::access;

    template <class Archive>
    void serialize(Archive& ar, const unsigned int version) {
        ar & boost::serialization::base_object<Metadata>(*this);
        ar & fields_;
    }

    std::map<std::string, const Metadata*> fields_;
};