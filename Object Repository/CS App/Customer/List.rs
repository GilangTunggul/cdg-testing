<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>List</name>
   <tag></tag>
   <elementGuidId>6e8b1f2e-676a-4399-adff-92f47714c944</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;function\&quot;: \&quot;list\&quot;,\n    \&quot;query\&quot;: {\n        \&quot;$match\&quot;: {\n            \&quot;$or\&quot;: [\n                {\n                    \&quot;name\&quot;: {\n                        \&quot;$regex\&quot;: \&quot;\&quot;,\n                        \&quot;$options\&quot;: \&quot;i\&quot;\n                    }\n                }\n            ]\n        },\n        \&quot;$sort\&quot;: [\n            {\n                \&quot;field\&quot;: \&quot;createdAt\&quot;,\n                \&quot;order\&quot;: \&quot;-1\&quot;\n            }\n        ],\n        \&quot;$skip\&quot;: \&quot;0\&quot;,\n        \&quot;$limit\&quot;: \&quot;8\&quot;\n    }\n}&quot;,
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
   <restUrl>${url}api/cdg/specialist/customers/1/list</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>0fab5cb8-f6c4-41df-97c5-bb79f9376166</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokencsapp</defaultValue>
      <description></description>
      <id>26490aaf-6bfa-4b65-b7b1-760d21e05f31</id>
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

WS.verifyElementPropertyValue(response, 'data', jsonResponse.data)

WS.verifyElementPropertyValue(response, 'message', 'success')

WS.verifyElementPropertyValue(response, 'data.data[0].companyID', '5948f33556d6c91479154da2')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
