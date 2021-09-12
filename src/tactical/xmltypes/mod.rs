pub mod xml_category;
pub mod xml_marker;
pub mod xml_route;
pub mod xml_trail;

pub const MARKER_SCHEMA_XSD: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified">
    <xs:element name="OverlayData">
        <xs:complexType>
            <xs:sequence>
                <xs:element ref="MarkerCategory" minOccurs="0" maxOccurs="1"/>
                <xs:element ref="POIs" minOccurs="0" maxOccurs="1"/>
            </xs:sequence>
        </xs:complexType>
    </xs:element>
    <xs:element name="POIs">
        <xs:complexType>
            <xs:choice maxOccurs="unbounded">
                <xs:element ref="POI"/>
                <xs:element ref="Trail"/>
            </xs:choice>
        </xs:complexType>
    </xs:element>
    <xs:element name="POI">
        <xs:complexType>
            <!-- region TacO defaults -->
            <xs:attribute name="GUID"/>
            <xs:attribute name="type" type="xs:NCName"/> <!-- This is not enforced by TacO -->
            <xs:attribute name="MapID" use="required" type="xs:integer"/>
            <xs:attribute name="xpos" use="required" type="xs:decimal"/>
            <xs:attribute name="ypos" use="required" type="xs:decimal"/>
            <xs:attribute name="zpos" use="required" type="xs:decimal"/>

            <!-- Icon -->
            <xs:attribute name="iconFile"/>
            <xs:attribute name="iconSize" type="xs:decimal"/>

            <!-- Optional -->
            <xs:attribute name="alpha" type="xs:decimal"/>
            <xs:attribute name="behaviour">
                <xs:simpleType>
                    <xs:restriction base="xs:nonNegativeInteger">
                        <xs:minInclusive value="0"/>
                        <xs:maxInclusive value="7"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="fadeNear" type="xs:decimal"/>
            <xs:attribute name="fadeFar" type="xs:decimal"/>
            <xs:attribute name="heightOffset" type="xs:decimal"/>
            <xs:attribute name="resetLength" type="xs:integer"/>
            <xs:attribute name="resetOffset" type="xs:integer"/>
            <xs:attribute name="DisplayName" type="xs:string"/>
            <xs:attribute name="color" type="xs:string"/> <!-- TODO: Verify it's a hex string -->
            <xs:attribute name="autoTrigger" type="xs:boolean"/>
            <xs:attribute name="hasCountdown" type="xs:boolean"/>
            <xs:attribute name="triggerRange" type="xs:decimal"/>
            <xs:attribute name="maxSize" type="xs:integer"/>
            <xs:attribute name="minSize" type="xs:integer"/>

            <!-- Achievement stuff -->
            <xs:attribute name="achievementId" type="xs:integer"/>
            <xs:attribute name="achievementBit" type="xs:integer"/>

            <!-- Minimap support -->
            <xs:attribute name="mapDisplaySize" type="xs:integer"/>
            <xs:attribute name="miniMapVisibility" type="xs:boolean"/>
            <xs:attribute name="mapVisibility" type="xs:boolean"/>
            <xs:attribute name="inGameVisibility" type="xs:boolean"/>
            <xs:attribute name="scaleOnMapWithZoom" type="xs:boolean"/>
            <xs:attribute name="mapFadeoutScaleLevel" type="xs:decimal"/>
            <xs:attribute name="keepOnMapEdge" type="xs:boolean"/>
            <!-- endregion -->

            <!-- region Blish -->
            <xs:attribute name="IsSeparator" type="xs:boolean"/>
            <xs:attribute name="defaultToggle" type="xs:boolean"/>
            <!-- Bounce -->
            <xs:attribute name="bounce-height" type="xs:decimal"/>
            <xs:attribute name="bounce-delay" type="xs:decimal"/>
            <xs:attribute name="bounce-duration" type="xs:decimal"/>
            <xs:attribute name="bounce-distance" type="xs:decimal"/>
            <xs:attribute name="bounce-when">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="always"/>
                        <xs:enumeration value="inzone"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>

            <!-- Copy -->
            <xs:attribute name="copy-radius" type="xs:integer"/>
            <xs:attribute name="copy-message" type="xs:string"/>

            <!-- Notification -->
            <xs:attribute name="notification" type="xs:string"/>
            <xs:attribute name="notification-distance" type="xs:decimal"/>
            <xs:attribute name="notification-type">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="info"/>
                        <xs:enumeration value="warning"/>
                        <xs:enumeration value="error"/>
                        <xs:enumeration value="gray"/>
                        <xs:enumeration value="blue"/>
                        <xs:enumeration value="green"/>
                        <xs:enumeration value="red"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>

            <!-- Rotate -->
            <xs:attribute name="rotate-x" type="xs:decimal"/>
            <xs:attribute name="rotate-y" type="xs:decimal"/>
            <xs:attribute name="rotate-z" type="xs:decimal"/>

            <!-- Title -->
            <xs:attribute name="title" type="xs:string"/>
            <xs:attribute name="title-color">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="white"/>
                        <xs:enumeration value="yellow"/>
                        <xs:enumeration value="red"/>
                        <xs:enumeration value="green"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>

            <!-- MumbleLink -->
            <xs:attribute name="mount">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value=""/> <!-- No mount -->
                        <xs:enumeration value="0"/> <!-- No mount -->
                        <xs:enumeration value="raptor"/>
                        <xs:enumeration value="springer"/>
                        <xs:enumeration value="skimmer"/>
                        <xs:enumeration value="jackal"/>
                        <xs:enumeration value="griffon"/>
                        <xs:enumeration value="rollerbeetle"/> <!-- TODO: Is this correct? -->
                        <xs:enumeration value="warclaw"/>
                        <xs:enumeration value="skyscale"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="race">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="asura"/>
                        <xs:enumeration value="charr"/>
                        <xs:enumeration value="human"/>
                        <xs:enumeration value="norn"/>
                        <xs:enumeration value="sylvari"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="profession">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="guardian"/>
                        <xs:enumeration value="revenant"/>
                        <xs:enumeration value="warrior"/>
                        <xs:enumeration value="engineer"/>
                        <xs:enumeration value="ranger"/>
                        <xs:enumeration value="thief"/>
                        <xs:enumeration value="elementalist"/>
                        <xs:enumeration value="mesmer"/>
                        <xs:enumeration value="necromancer"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="specialization">
                <xs:simpleType>
                    <xs:restriction base="xs:integer">
                        <xs:enumeration value="5"/> <!-- Druid -->
                        <xs:enumeration value="7"/> <!-- Daredevil -->
                        <xs:enumeration value="18"/> <!-- Berserker -->
                        <xs:enumeration value="27"/> <!-- Dragonhunter -->
                        <xs:enumeration value="34"/> <!-- Reaper -->
                        <xs:enumeration value="40"/> <!-- Chronomancer -->
                        <xs:enumeration value="43"/> <!-- Scrapper -->
                        <xs:enumeration value="48"/> <!-- Tempest -->
                        <xs:enumeration value="52"/> <!-- Herald -->
                        <xs:enumeration value="55"/> <!-- Soulbeast -->
                        <xs:enumeration value="56"/> <!-- Weaver -->
                        <xs:enumeration value="57"/> <!-- Holosmith -->
                        <xs:enumeration value="58"/> <!-- Deadeye -->
                        <xs:enumeration value="59"/> <!-- Mirage -->
                        <xs:enumeration value="60"/> <!-- Scourge -->
                        <xs:enumeration value="61"/> <!-- Spellbreaker -->
                        <xs:enumeration value="62"/> <!-- Firebrand -->
                        <xs:enumeration value="63"/> <!-- Renegade -->
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <!-- endregion -->
        </xs:complexType>
    </xs:element>
    <xs:element name="Trail">
        <xs:complexType>
            <!-- TacO defaults -->
            <xs:attribute name="GUID"/>

            <xs:attribute name="color" type="xs:string"/> <!-- TODO: Verify it's a hex string -->
            <xs:attribute name="type" use="required" type="xs:NCName"/>
            <xs:attribute name="alpha" type="xs:decimal"/>
            <xs:attribute name="fadeNear" type="xs:decimal"/>
            <xs:attribute name="fadeFar" type="xs:decimal"/>

            <xs:attribute name="trailData" use="required" type="xs:string"/>
            <xs:attribute name="texture" use="required" type="xs:string"/>
            <xs:attribute name="animSpeed" type="xs:decimal"/>
            <xs:attribute name="trailScale" type="xs:decimal"/>

            <!-- Blish? -->
<!--            <xs:attribute name="heightOffset" type="xs:decimal"/>-->
            <xs:attribute name="maxSize" type="xs:integer"/>
            <xs:attribute name="minSize" type="xs:integer"/>
        </xs:complexType>
    </xs:element>
    <xs:element name="MarkerCategory">
        <xs:complexType>
            <xs:sequence>
                <xs:element minOccurs="0" maxOccurs="unbounded" ref="MarkerCategory"/>
            </xs:sequence>

            <!-- region TacO defaults -->
            <xs:attribute name="name" use="required" type="xs:NCName"/>

            <!-- Icon -->
            <xs:attribute name="iconFile"/>
            <xs:attribute name="iconSize" type="xs:decimal"/>

            <!-- Optional -->
            <xs:attribute name="alpha" type="xs:decimal"/>
            <xs:attribute name="behaviour">
                <xs:simpleType>
                    <xs:restriction base="xs:nonNegativeInteger">
                        <xs:minInclusive value="0"/>
                        <xs:maxInclusive value="7"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="fadeNear" type="xs:decimal"/>
            <xs:attribute name="fadeFar" type="xs:decimal"/>
            <xs:attribute name="heightOffset" type="xs:decimal"/>
            <xs:attribute name="resetLength" type="xs:integer"/>
            <xs:attribute name="resetOffset" type="xs:integer"/>
            <xs:attribute name="DisplayName" type="xs:string"/>
            <xs:attribute name="color" type="xs:string"/> <!-- TODO: Verify it's a hex string -->
            <xs:attribute name="autoTrigger" type="xs:boolean"/>
            <xs:attribute name="hasCountdown" type="xs:boolean"/>
            <xs:attribute name="triggerRange" type="xs:decimal"/>
            <xs:attribute name="maxSize" type="xs:integer"/>
            <xs:attribute name="minSize" type="xs:integer"/>

            <!-- Achievement stuff -->
            <xs:attribute name="achievementId" type="xs:integer"/>
            <xs:attribute name="achievementBit" type="xs:integer"/>

            <!-- Minimap support -->
            <xs:attribute name="mapDisplaySize" type="xs:integer"/>
            <xs:attribute name="miniMapVisibility" type="xs:boolean"/>
            <xs:attribute name="mapVisibility" type="xs:boolean"/>
            <xs:attribute name="inGameVisibility" type="xs:boolean"/>
            <xs:attribute name="scaleOnMapWithZoom" type="xs:boolean"/>
            <xs:attribute name="mapFadeoutScaleLevel" type="xs:decimal"/>
            <xs:attribute name="keepOnMapEdge" type="xs:boolean"/>
            <!-- endregion -->

            <!-- region Blish -->
            <xs:attribute name="IsSeparator" type="xs:boolean"/>
            <xs:attribute name="defaultToggle" type="xs:boolean"/>
            <!-- Bounce -->
            <xs:attribute name="bounce-height" type="xs:decimal"/>
            <xs:attribute name="bounce-delay" type="xs:decimal"/>
            <xs:attribute name="bounce-duration" type="xs:decimal"/>
            <xs:attribute name="bounce-distance" type="xs:decimal"/>
            <xs:attribute name="bounce-when">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="always"/>
                        <xs:enumeration value="inzone"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>

            <!-- Copy -->
            <xs:attribute name="copy-radius" type="xs:integer"/>
            <xs:attribute name="copy-message" type="xs:string"/>

            <!-- Notification -->
            <xs:attribute name="notification" type="xs:string"/>
            <xs:attribute name="notification-distance" type="xs:decimal"/>
            <xs:attribute name="notification-type">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="info"/>
                        <xs:enumeration value="warning"/>
                        <xs:enumeration value="error"/>
                        <xs:enumeration value="gray"/>
                        <xs:enumeration value="blue"/>
                        <xs:enumeration value="green"/>
                        <xs:enumeration value="red"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>

            <!-- Rotate -->
            <xs:attribute name="rotate-x" type="xs:decimal"/>
            <xs:attribute name="rotate-y" type="xs:decimal"/>
            <xs:attribute name="rotate-z" type="xs:decimal"/>

            <!-- Title -->
            <xs:attribute name="title" type="xs:string"/>
            <xs:attribute name="title-color">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="white"/>
                        <xs:enumeration value="yellow"/>
                        <xs:enumeration value="red"/>
                        <xs:enumeration value="green"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>

            <!-- MumbleLink -->
            <xs:attribute name="mount">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value=""/> <!-- No mount -->
                        <xs:enumeration value="0"/> <!-- No mount -->
                        <xs:enumeration value="raptor"/>
                        <xs:enumeration value="springer"/>
                        <xs:enumeration value="skimmer"/>
                        <xs:enumeration value="jackal"/>
                        <xs:enumeration value="griffon"/>
                        <xs:enumeration value="rollerbeetle"/> <!-- TODO: Is this correct? -->
                        <xs:enumeration value="warclaw"/>
                        <xs:enumeration value="skyscale"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="race">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="asura"/>
                        <xs:enumeration value="charr"/>
                        <xs:enumeration value="human"/>
                        <xs:enumeration value="norn"/>
                        <xs:enumeration value="sylvari"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="profession">
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="guardian"/>
                        <xs:enumeration value="revenant"/>
                        <xs:enumeration value="warrior"/>
                        <xs:enumeration value="engineer"/>
                        <xs:enumeration value="ranger"/>
                        <xs:enumeration value="thief"/>
                        <xs:enumeration value="elementalist"/>
                        <xs:enumeration value="mesmer"/>
                        <xs:enumeration value="necromancer"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="specialization">
                <xs:simpleType>
                    <xs:restriction base="xs:integer">
                        <xs:enumeration value="5"/> <!-- Druid -->
                        <xs:enumeration value="7"/> <!-- Daredevil -->
                        <xs:enumeration value="18"/> <!-- Berserker -->
                        <xs:enumeration value="27"/> <!-- Dragonhunter -->
                        <xs:enumeration value="34"/> <!-- Reaper -->
                        <xs:enumeration value="40"/> <!-- Chronomancer -->
                        <xs:enumeration value="43"/> <!-- Scrapper -->
                        <xs:enumeration value="48"/> <!-- Tempest -->
                        <xs:enumeration value="52"/> <!-- Herald -->
                        <xs:enumeration value="55"/> <!-- Soulbeast -->
                        <xs:enumeration value="56"/> <!-- Weaver -->
                        <xs:enumeration value="57"/> <!-- Holosmith -->
                        <xs:enumeration value="58"/> <!-- Deadeye -->
                        <xs:enumeration value="59"/> <!-- Mirage -->
                        <xs:enumeration value="60"/> <!-- Scourge -->
                        <xs:enumeration value="61"/> <!-- Spellbreaker -->
                        <xs:enumeration value="62"/> <!-- Firebrand -->
                        <xs:enumeration value="63"/> <!-- Renegade -->
                    </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
            <!-- endregion -->
        </xs:complexType>
    </xs:element>
</xs:schema>
"#;
