<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Ticket list</name>
   <tag></tag>
   <elementGuidId>7716e6af-7a22-4af7-b369-63c383ad45e5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;function\&quot;: \&quot;list\&quot;,\n    \&quot;query\&quot;: {\n        \&quot;$match\&quot;: {\n            \&quot;$or\&quot;: [\n                {\n                    \&quot;productName\&quot;: {\n                        \&quot;$regex\&quot;: \&quot;\&quot;,\n                        \&quot;$options\&quot;: \&quot;i\&quot;\n                    }\n                },\n                {\n                    \&quot;ticketNumber\&quot;: {\n                        \&quot;$regex\&quot;: \&quot;\&quot;,\n                        \&quot;$options\&quot;: \&quot;i\&quot;\n                    }\n                }\n            ]\n        },\n        \&quot;$sort\&quot;: [\n            {\n                \&quot;field\&quot;: \&quot;createdAt\&quot;,\n                \&quot;order\&quot;: \&quot;-1\&quot;\n            }\n        ],\n        \&quot;$skip\&quot;: \&quot;0\&quot;,\n        \&quot;$limit\&quot;: \&quot;8\&quot;\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}api/cdg/specialist/tickets/lists</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>89ef11bb-e083-48b4-8315-02c05cc13d9a</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokencsapp</defaultValue>
      <description></description>
      <id>105f8176-5d31-472b-9b5c-388b646f73d9</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

JsonSlurper slurper = new JsonSlurper()

Map parsedJson = slurper.parseText(response.getResponseBodyContent())

println(jsonResponse.data != null)

WS.verifyElementPropertyValue(response, 'data.data', jsonResponse.data.data)

String B = parsedJson.get(&quot;_id&quot;)

WS.verifyElementPropertyValue(response, 'message', 'success')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
