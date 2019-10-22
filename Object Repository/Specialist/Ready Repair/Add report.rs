<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add report</name>
   <tag></tag>
   <elementGuidId>9a51763e-5011-4355-9f80-5345a3785c94</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;serviceID&quot;,
      &quot;value&quot;: &quot;${id}&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;categories[]&quot;,
      &quot;value&quot;: &quot;5d144b30c7c95d2fa95fb424&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;result&quot;,
      &quot;value&quot;: &quot;ss&quot;,
      &quot;type&quot;: &quot;Text&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}module/cdg/main/specialist/product/repairReport</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>867c99da-f5be-4962-b684-bce1f1a00017</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>a72494b8-9fd7-4427-8c53-5f98dce119cc</id>
      <masked>false</masked>
      <name>cookies</name>
   </variables>
   <variables>
      <defaultValue>'5d1ad2f3c7c95d12c14c0652'</defaultValue>
      <description></description>
      <id>63ba4355-cd2d-4093-9dd6-73273a030f1b</id>
      <masked>false</masked>
      <name>id</name>
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

//def jsonSlurper = new JsonSlurper()
//
//def jsonResponse = jsonSlurper.parseText(response.getResponseText())
//
//def variables = request.getVariables()
//
//def id = variables.get('id')
//
//println(jsonResponse.data)
//
//println(jsonResponse.message)
//
//println(jsonResponse.success)
//
//WS.verifyElementPropertyValue(response, 'data', id)
//
//WS.verifyElementPropertyValue(response, 'message', '[]')
//
//WS.verifyElementPropertyValue(response, 'success', true)


JsonSlurper slurper = new JsonSlurper()

Map parsedJson = slurper.parseText(response.getResponseText())

parsedJson.get('data')

parsedJson.get('message')

parsedJson.get('success')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
