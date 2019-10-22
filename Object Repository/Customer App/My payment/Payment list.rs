<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Payment list</name>
   <tag></tag>
   <elementGuidId>89b4f7b0-7958-4074-a049-c2d823031a11</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;function\&quot;: \&quot;list\&quot;,\n  \&quot;query\&quot;: {\n      \&quot;$match\&quot;: {\n      \t \&quot;status\&quot;: \&quot;unpaid\&quot;\n      },\n      \&quot;$sort\&quot;: [\n          {\n              \&quot;field\&quot;: \&quot;createdAt\&quot;,\n              \&quot;order\&quot;: \&quot;-1\&quot;\n          }\n      ],\n      \&quot;$skip\&quot;: \&quot;0\&quot;,\n      \&quot;$limit\&quot;: \&quot;8\&quot;\n  }\n}&quot;,
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
   <restUrl>${url}api/cdg/customer/payment/list</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>a6b5932c-726c-4c54-af1d-1380d008c2b6</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>3d2a385c-8420-4425-a912-d870962e9796</id>
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

println(jsonResponse.data != null)

WS.verifyElementPropertyValue(response, 'data', jsonResponse.data)

WS.verifyElementPropertyValue(response, 'message', 'success')

WS.verifyElementPropertyValue(response, 'data.data[0].status', 'unpaid')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
